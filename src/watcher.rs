use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use tokio::sync::broadcast;

/// 監視対象から除外すべきディレクトリやファイルパターンをチェック
fn should_ignore_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // 除外すべきディレクトリ
    let ignore_dirs = [
        ".git",
        ".svn",
        ".hg",
        "node_modules",
        "target",
        ".venv",
        "venv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        ".cargo",
        ".idea",
        ".vscode",
    ];

    // 除外すべきファイルパターン
    let ignore_patterns = [
        ".lock",
        ".swp",
        ".swo",
        ".tmp",
        "~",
        ".DS_Store",
    ];

    // パスに除外ディレクトリが含まれているかチェック
    for dir in &ignore_dirs {
        if path_str.contains(&format!("/{}/", dir)) || path_str.ends_with(&format!("/{}", dir)) {
            return true;
        }
    }

    // ファイル名が除外パターンに一致するかチェック
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        for pattern in &ignore_patterns {
            if file_name.ends_with(pattern) || file_name.starts_with('.') && file_name.contains(pattern) {
                return true;
            }
        }
    }

    false
}

pub fn start_watcher(
    watch_path: PathBuf,
) -> Result<(RecommendedWatcher, broadcast::Receiver<()>), Box<dyn std::error::Error>> {
    let (tx, rx) = broadcast::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                // ファイルの変更イベントを検知
                if matches!(
                    event.kind,
                    notify::EventKind::Modify(_) | notify::EventKind::Create(_)
                ) {
                    // 無視すべきパスかチェック
                    let should_ignore = event.paths.iter().all(|path| should_ignore_path(path));

                    if should_ignore {
                        // デバッグ用：無視されたファイルをログに出力（必要に応じてコメントアウト）
                        eprintln!("[RELOAD] Ignored: {:?}", event.paths);
                        return;
                    }

                    // デバッグ用：有効な変更をログに出力
                    eprintln!("[RELOAD] Event detected: {:?}, paths: {:?}", event.kind, event.paths);
                    let _ = tx.send(());
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    Ok((watcher, rx))
}
