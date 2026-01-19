use crate::fs::{FileEntry, AppError};
use std::path::PathBuf;

#[tauri::command]
pub async fn read_directory(path: String) -> Result<Vec<FileEntry>, AppError> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err(AppError::NotFound(path.display().to_string()));
    }

    if !path.is_dir() {
        return Err(AppError::NotADirectory(path.display().to_string()));
    }

    let mut entries = Vec::new();

    let read_dir = std::fs::read_dir(&path)
        .map_err(|e| AppError::Io(e.to_string()))?;

    for entry in read_dir {
        let entry = entry.map_err(|e| AppError::Io(e.to_string()))?;
        let metadata = entry.metadata().map_err(|e| AppError::Io(e.to_string()))?;
        let file_name = entry.file_name().to_string_lossy().to_string();

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

        entries.push(FileEntry {
            name: file_name,
            path: entry.path().display().to_string(),
            extension,
            size: if metadata.is_file() { Some(metadata.len()) } else { None },
            modified,
            is_dir: metadata.is_dir(),
            is_hidden,
            is_symlink: metadata.is_symlink(),
        });
    }

    // Sort: directories first, then by name
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
}

#[tauri::command]
pub async fn get_parent_directory(path: String) -> Option<String> {
    PathBuf::from(&path)
        .parent()
        .map(|p| p.display().to_string())
}

#[tauri::command]
pub async fn get_home_directory() -> Option<String> {
    dirs::home_dir().map(|p| p.display().to_string())
}

#[tauri::command]
pub async fn open_file(path: String) -> Result<(), AppError> {
    open::that(&path).map_err(|e| AppError::Io(e.to_string()))
}
