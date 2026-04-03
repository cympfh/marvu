use std::io::{Read, Seek};
use std::path::Path;
use zip::ZipArchive;

pub struct ZipEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}

pub fn is_zip_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("zip"))
        .unwrap_or(false)
}

pub fn list_zip_contents(zip_path: &Path) -> Result<Vec<ZipEntry>, String> {
    let file = std::fs::File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    let mut entries = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| format!("Failed to read zip entry: {}", e))?;

        let name = file.name().to_string();
        let is_dir = file.is_dir();
        let size = file.size();

        entries.push(ZipEntry {
            name,
            is_dir,
            size,
        });
    }

    Ok(entries)
}

pub fn extract_file_from_zip<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    file_path: &str,
) -> Result<Vec<u8>, String> {
    let mut file = archive
        .by_name(file_path)
        .map_err(|e| format!("File not found in zip: {}", e))?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| format!("Failed to read file from zip: {}", e))?;

    Ok(contents)
}

pub fn read_file_from_zip(zip_path: &Path, file_path: &str) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(zip_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip archive: {}", e))?;

    extract_file_from_zip(&mut archive, file_path)
}

pub fn get_directory_entries(entries: &[ZipEntry], dir_path: &str) -> Vec<ZipEntry> {
    let prefix = if dir_path.is_empty() {
        String::new()
    } else {
        format!("{}/", dir_path)
    };

    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for entry in entries {
        if !entry.name.starts_with(&prefix) {
            continue;
        }

        let relative = &entry.name[prefix.len()..];

        // ディレクトリ直下のエントリのみ
        if let Some(slash_pos) = relative.find('/') {
            let dir_name = &relative[..slash_pos];
            if !seen.contains(dir_name) {
                seen.insert(dir_name.to_string());
                result.push(ZipEntry {
                    name: format!("{}{}", prefix, dir_name),
                    is_dir: true,
                    size: 0,
                });
            }
        } else if !relative.is_empty() {
            result.push(ZipEntry {
                name: entry.name.clone(),
                is_dir: entry.is_dir,
                size: entry.size,
            });
        }
    }

    result.sort_by(|a, b| {
        // ディレクトリを先に、その後名前でソート
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zip_file() {
        assert!(is_zip_file(Path::new("test.zip")));
        assert!(is_zip_file(Path::new("test.ZIP")));
        assert!(!is_zip_file(Path::new("test.txt")));
        assert!(!is_zip_file(Path::new("test")));
    }
}
