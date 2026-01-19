use super::{AppError, UndoToken};
use std::path::PathBuf;
use uuid::Uuid;

pub struct FileOperations;

impl FileOperations {
    pub async fn copy_files(sources: Vec<String>, dest: String) -> Result<(), AppError> {
        let dest_path = PathBuf::from(&dest);

        if !dest_path.exists() {
            return Err(AppError::NotFound(dest));
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

    pub async fn move_files(sources: Vec<String>, dest: String) -> Result<(), AppError> {
        let dest_path = PathBuf::from(&dest);

        if !dest_path.exists() {
            return Err(AppError::NotFound(dest));
        }

        for source in sources {
            let source_path = PathBuf::from(&source);
            let file_name = source_path
                .file_name()
                .ok_or_else(|| AppError::InvalidOperation("Invalid source path".into()))?;

            let target = dest_path.join(file_name);

            std::fs::rename(&source_path, &target)
                .map_err(|e| AppError::Io(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn delete_files(paths: Vec<String>) -> Result<UndoToken, AppError> {
        let token = UndoToken {
            id: Uuid::new_v4().to_string(),
            operation: "delete".to_string(),
            paths: paths.clone(),
            backup_paths: Vec::new(),
        };

        for path in &paths {
            let path = PathBuf::from(path);

            if path.is_dir() {
                trash::delete(&path).map_err(|e| AppError::Io(e.to_string()))?;
            } else {
                trash::delete(&path).map_err(|e| AppError::Io(e.to_string()))?;
            }
        }

        Ok(token)
    }

    pub async fn create_directory(path: String, name: String) -> Result<String, AppError> {
        let new_path = PathBuf::from(&path).join(&name);

        if new_path.exists() {
            return Err(AppError::InvalidOperation(format!(
                "Directory already exists: {}",
                name
            )));
        }

        std::fs::create_dir(&new_path).map_err(|e| AppError::Io(e.to_string()))?;

        Ok(new_path.display().to_string())
    }

    pub async fn rename(path: String, new_name: String) -> Result<String, AppError> {
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
