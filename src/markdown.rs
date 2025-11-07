use std::path::PathBuf;
use tokio::process::Command;

const RELOAD_HTML: &str = r#"<script src="/__reload__.js"></script>
<style>
* { box-sizing: border-box; }
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
    background: #ffffff;
    min-height: 100vh;
    padding: 2rem 1rem;
    line-height: 1.7;
    color: #333;
    max-width: 850px;
    margin: 0 auto;
}
@keyframes fadeIn {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}
h1, h2, h3, h4, h5, h6 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    font-weight: 700;
    line-height: 1.3;
    color: #667eea;
}
h1 {
    font-size: 2.5rem;
    border-bottom: 3px solid #667eea;
    padding-bottom: 0.5rem;
}
h2 {
    font-size: 2rem;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 0.4rem;
}
h3 { font-size: 1.5rem; }
h4 { font-size: 1.25rem; }
p { margin: 1rem 0; }
a {
    color: #667eea;
    text-decoration: none;
    border-bottom: 1px solid transparent;
    transition: all 0.2s ease;
}
a:hover {
    border-bottom-color: #667eea;
    color: #764ba2;
}
code {
    background: #f3f4f6;
    padding: 0.2em 0.4em;
    border-radius: 6px;
    font-family: 'Fira Code', 'Consolas', 'Monaco', monospace;
    font-size: 0.9em;
    color: #e83e8c;
}
pre {
    background: #282c34;
    color: #abb2bf;
    padding: 1.5rem;
    border-radius: 12px;
    overflow-x: auto;
    margin: 1.5rem 0;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
pre code {
    background: none;
    color: inherit;
    padding: 0;
    font-size: 0.95em;
}
blockquote {
    border-left: 4px solid #667eea;
    padding-left: 1.5rem;
    margin: 1.5rem 0;
    color: #555;
    background: #f9fafb;
    padding: 1rem 1.5rem;
    border-radius: 0 8px 8px 0;
}
ul, ol {
    padding-left: 2rem;
    margin: 1rem 0;
}
li { margin: 0.5rem 0; }
table {
    border-collapse: collapse;
    width: 100%;
    margin: 1.5rem 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    overflow: hidden;
}
th, td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
}
th {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 600;
}
tr:hover {
    background: #f9fafb;
}
img {
    max-width: 100%;
    height: auto;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
}
hr {
    border: none;
    border-top: 2px solid #e5e7eb;
    margin: 2rem 0;
}
@media (max-width: 768px) {
    body { padding: 1rem 0.5rem; }
    .markdown-body { padding: 1.5rem; }
    h1 { font-size: 2rem; }
    h2 { font-size: 1.5rem; }
}
</style>"#;

pub async fn convert_to_html(file_path: &PathBuf) -> Result<String, String> {
    // 一時ファイルにreload.htmlを書き出し
    let temp_html = std::env::temp_dir().join("grow_reload.html");
    std::fs::write(&temp_html, RELOAD_HTML)
        .map_err(|e| format!("Failed to write reload.html: {}", e))?;

    // コマンドをログ出力
    eprintln!(
        "[unidoc] Running: unidoc -s -H {} {}",
        temp_html.display(),
        file_path.display()
    );

    let output = Command::new("unidoc")
        .arg("-s")
        .arg("-H")
        .arg(&temp_html)
        .arg("--")
        .arg(file_path)
        .output()
        .await
        .map_err(|e| format!("Failed to execute unidoc: {}", e))?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in output: {}", e))
    } else {
        Err(format!(
            "unidoc failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
