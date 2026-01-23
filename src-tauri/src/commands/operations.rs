use crate::fs::{AppError, UndoToken};
use std::path::PathBuf;

#[tauri::command]
pub async fn copy_files(sources: Vec<String>, dest: String) -> Result<(), AppError> {
    let dest_path = PathBuf::from(&dest);

    if !dest_path.exists() {
        return Err(AppError::NotFound(dest));
    }

    if !dest_path.is_dir() {
        return Err(AppError::NotADirectory(dest));
    }

    for source in sources {
        let source_path = PathBuf::from(&source);
        let file_name = source_path
            .file_name()
            .ok_or_else(|| AppError::InvalidOperation("Invalid source path".into()))?;

        let target = dest_path.join(file_name);

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &target)?;
        } else {
            std::fs::copy(&source_path, &target)
                .map_err(|e| AppError::Io(e.to_string()))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn move_files(sources: Vec<String>, dest: String) -> Result<(), AppError> {
    let dest_path = PathBuf::from(&dest);

    if !dest_path.exists() {
        return Err(AppError::NotFound(dest));
    }

    if !dest_path.is_dir() {
        return Err(AppError::NotADirectory(dest));
    }

    for source in sources {
        let source_path = PathBuf::from(&source);
        let file_name = source_path
            .file_name()
            .ok_or_else(|| AppError::InvalidOperation("Invalid source path".into()))?;

        let target = dest_path.join(file_name);

        // Try rename first (faster for same filesystem)
        if std::fs::rename(&source_path, &target).is_err() {
            // Fall back to copy + delete for cross-filesystem moves
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &target)?;
                std::fs::remove_dir_all(&source_path)
                    .map_err(|e| AppError::Io(e.to_string()))?;
            } else {
                std::fs::copy(&source_path, &target)
                    .map_err(|e| AppError::Io(e.to_string()))?;
                std::fs::remove_file(&source_path)
                    .map_err(|e| AppError::Io(e.to_string()))?;
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_files(paths: Vec<String>) -> Result<UndoToken, AppError> {
    let token = UndoToken {
        id: uuid::Uuid::new_v4().to_string(),
        operation: "delete".to_string(),
        paths: paths.clone(),
        backup_paths: Vec::new(),
    };

    for path in &paths {
        let path = PathBuf::from(path);
        trash::delete(&path).map_err(|e| AppError::Io(e.to_string()))?;
    }

    Ok(token)
}

#[tauri::command]
pub async fn create_directory(parent_path: String, name: String) -> Result<String, AppError> {
    let new_path = PathBuf::from(&parent_path).join(&name);

    if new_path.exists() {
        return Err(AppError::InvalidOperation(format!(
            "Directory already exists: {}",
            name
        )));
    }

    std::fs::create_dir(&new_path).map_err(|e| AppError::Io(e.to_string()))?;

    Ok(new_path.display().to_string())
}

#[tauri::command]
pub async fn rename_file(path: String, new_name: String) -> Result<String, AppError> {
    let source = PathBuf::from(&path);
    let parent = source
        .parent()
        .ok_or_else(|| AppError::InvalidOperation("Cannot rename root".into()))?;

    let target = parent.join(&new_name);

    if target.exists() {
        return Err(AppError::InvalidOperation(format!(
            "File already exists: {}",
            new_name
        )));
    }

    std::fs::rename(&source, &target).map_err(|e| AppError::Io(e.to_string()))?;

    Ok(target.display().to_string())
}

#[tauri::command]
pub async fn get_file_info(path: String) -> Result<crate::fs::FileEntry, AppError> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err(AppError::NotFound(path.display().to_string()));
    }

    let metadata = std::fs::metadata(&path)
        .map_err(|e| AppError::Io(e.to_string()))?;

    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let extension = if metadata.is_file() {
        path.extension().map(|e| e.to_string_lossy().to_string())
    } else {
        None
    };

    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    Ok(crate::fs::FileEntry {
        name: file_name.clone(),
        path: path.display().to_string(),
        extension,
        size: if metadata.is_file() { Some(metadata.len()) } else { None },
        modified,
        is_dir: metadata.is_dir(),
        is_hidden: file_name.starts_with('.'),
        is_symlink: metadata.is_symlink(),
    })
}

fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<(), AppError> {
    std::fs::create_dir_all(dest).map_err(|e| AppError::Io(e.to_string()))?;

    for entry in std::fs::read_dir(src).map_err(|e| AppError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::Io(e.to_string()))?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            std::fs::copy(&src_path, &dest_path).map_err(|e| AppError::Io(e.to_string()))?;
        }
    }

    Ok(())
}
