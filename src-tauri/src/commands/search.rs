use crate::fs::{FileEntry, AppError};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DriveInfo {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub async fn search_files(
    query: String,
    root_paths: Vec<String>,
    limit: Option<usize>,
) -> Result<Vec<FileEntry>, AppError> {
    let limit = limit.unwrap_or(500);
    let query_lower = query.to_lowercase();

    if query_lower.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    for root in root_paths {
        let root_path = PathBuf::from(&root);

        if !root_path.exists() {
            continue;
        }

        for entry in WalkDir::new(&root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if results.len() >= limit {
                break;
            }

            let file_name = entry.file_name().to_string_lossy().to_string();

            // Case-insensitive substring match on filename
            if file_name.to_lowercase().contains(&query_lower) {
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let extension = if metadata.is_file() {
                    PathBuf::from(&file_name)
                        .extension()
                        .map(|e| e.to_string_lossy().to_string())
                } else {
                    None
                };

                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs() as i64);

                let is_hidden = file_name.starts_with('.');

                results.push(FileEntry {
                    name: file_name,
                    path: entry.path().display().to_string(),
                    extension,
                    size: if metadata.is_file() { Some(metadata.len()) } else { None },
                    modified,
                    is_dir: metadata.is_dir(),
                    is_hidden,
                    is_symlink: metadata.file_type().is_symlink(),
                });
            }
        }

        if results.len() >= limit {
            break;
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_available_drives() -> Vec<DriveInfo> {
    let mut drives = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Check A-Z drives on Windows
        for letter in b'A'..=b'Z' {
            let drive_letter = letter as char;
            let path = format!("{}:\\", drive_letter);
            let path_buf = PathBuf::from(&path);

            if path_buf.exists() {
                drives.push(DriveInfo {
                    name: format!("{}: Drive", drive_letter),
                    path,
                });
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Common Linux/Unix paths
        let paths = ["/", "/home"];
        for p in paths {
            let path_buf = PathBuf::from(p);
            if path_buf.exists() {
                drives.push(DriveInfo {
                    name: p.to_string(),
                    path: p.to_string(),
                });
            }
        }
    }

    drives
}
