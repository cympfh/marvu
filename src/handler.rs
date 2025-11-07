use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response, Sse},
    response::sse::{Event, KeepAlive},
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::markdown;

#[derive(Clone)]
pub struct AppState {
    pub base_dir: Arc<PathBuf>,
    pub reload_tx: broadcast::Sender<()>,
}

pub async fn handle_root(State(state): State<AppState>) -> Response {
    handle_directory(&state.base_dir, "".to_string()).await
}

pub async fn handle_path(State(state): State<AppState>, Path(path): Path<String>) -> Response {
    let full_path = state.base_dir.join(&path);

    // セキュリティチェック: base_dir外へのアクセスを防ぐ
    if !full_path.starts_with(&*state.base_dir) {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }

    if !full_path.exists() {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    }

    if full_path.is_dir() {
        handle_directory(&full_path, path).await
    } else {
        handle_file(&full_path, &path).await
    }
}

async fn handle_directory(dir_path: &PathBuf, relative_path: String) -> Response {
    let mut entries = match tokio::fs::read_dir(dir_path).await {
        Ok(entries) => entries,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot read directory").into_response()
        }
    };

    let mut html = String::from(r#"<!DOCTYPE html>
<html><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Directory Listing</title>
<script src="/__reload__.js"></script>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    padding: 2rem;
    color: #333;
}
.container {
    max-width: 900px;
    margin: 0 auto;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    padding: 2.5rem;
    animation: fadeIn 0.5s ease;
}
@keyframes fadeIn {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}
h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 1.5rem;
    color: #667eea;
    border-bottom: 3px solid #667eea;
    padding-bottom: 0.75rem;
    word-break: break-all;
}
.path {
    font-size: 1rem;
    color: #666;
    font-weight: 400;
}
ul {
    list-style: none;
}
li {
    border-bottom: 1px solid #e5e7eb;
}
li:last-child {
    border-bottom: none;
}
a {
    display: flex;
    align-items: center;
    padding: 1rem 1.25rem;
    text-decoration: none;
    color: #374151;
    font-size: 1rem;
    transition: all 0.2s ease;
    border-radius: 8px;
}
a:hover {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    transform: translateX(8px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}
.icon {
    margin-right: 1rem;
    font-size: 1.5rem;
    min-width: 1.5rem;
}
.dir { color: #667eea; }
.file { color: #9ca3af; }
a.markdown { font-weight: 600; }
a:hover .icon { transform: scale(1.2); transition: transform 0.2s; }
.parent {
    font-weight: 600;
    color: #667eea;
}
@media (max-width: 640px) {
    body { padding: 1rem; }
    .container { padding: 1.5rem; }
    h1 { font-size: 1.5rem; }
}
</style>
</head><body><div class="container">"#);

    html.push_str(&format!(
        "<h1><span class=\"path\">📁 /{}</span></h1>",
        if relative_path.is_empty() { "Home".to_string() } else { relative_path.clone() }
    ));
    html.push_str("<ul>");

    // 親ディレクトリへのリンク
    if !relative_path.is_empty() {
        let parent = if relative_path.contains('/') {
            relative_path.rsplitn(2, '/').nth(1).unwrap()
        } else {
            ""
        };
        html.push_str(&format!(
            "<li><a href=\"/{}\" class=\"parent\"><span class=\"icon\">⬆️</span>Parent Directory</a></li>",
            parent
        ));
    }

    let mut items = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        if let Ok(file_name) = entry.file_name().into_string() {
            let is_dir = entry.path().is_dir();
            items.push((file_name, is_dir));
        }
    }
    items.sort();

    for (name, is_dir) in items {
        let link_path = if relative_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", relative_path, name)
        };

        // Markdownファイルには特別なアイコンを使用
        let (icon, icon_class, link_class) = if is_dir {
            ("📁", "dir", "")
        } else if name.ends_with(".md") || name.ends_with(".mkd") {
            ("📝", "file", " markdown")
        } else {
            ("📄", "file", "")
        };

        html.push_str(&format!(
            "<li><a href=\"/{}\" class=\"{}\"><span class=\"icon {}\">{}</span>{}</a></li>",
            link_path, link_class.trim(), icon_class, icon, name
        ));
    }

    html.push_str("</ul></div></body></html>");
    Html(html).into_response()
}

async fn handle_file(file_path: &PathBuf, _relative_path: &str) -> Response {
    let extension = file_path.extension().and_then(|s| s.to_str());

    // マークダウンファイルの場合はunidocで変換
    if matches!(extension, Some("md") | Some("mkd")) {
        match markdown::convert_to_html(file_path).await {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Markdown conversion failed: {}", e),
            )
                .into_response(),
        }
    } else {
        // その他のファイルはそのまま返す
        match tokio::fs::read(file_path).await {
            Ok(contents) => contents.into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cannot read file").into_response(),
        }
    }
}

pub async fn handle_reload_events(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.reload_tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|_| Ok(Event::default().data("reload")));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn handle_reload_js() -> Response {
    let js = include_str!("reload.js");
    (
        StatusCode::OK,
        [("Content-Type", "application/javascript")],
        js,
    )
        .into_response()
}
