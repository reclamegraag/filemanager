use super::entry::IndexEntry;
use super::IndexManager;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::AppHandle;
use walkdir::WalkDir;

const PROGRESS_BATCH_SIZE: usize = 5000;

pub async fn scan_directories(
    manager: Arc<IndexManager>,
    roots: Vec<PathBuf>,
    app: AppHandle,
) {
    manager.clear();
    manager.set_roots(roots.clone());
    manager.reset_stop();
    manager.set_status(super::entry::IndexStatus::Scanning, Some(&app));

    let mut batch = Vec::with_capacity(PROGRESS_BATCH_SIZE);
    let mut total_count: usize = 0;

    for root in roots {
        if manager.should_stop() {
            break;
        }

        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if manager.should_stop() {
                break;
            }

            let path = entry.path().to_path_buf();
            let file_name = entry.file_name().to_string_lossy().to_string();

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

            let size = if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            };

            let index_entry = IndexEntry::new(
                file_name,
                extension,
                metadata.is_dir(),
                modified,
                size,
            );

            batch.push((path, index_entry));
            total_count += 1;

            if batch.len() >= PROGRESS_BATCH_SIZE {
                let current_path = entry.path().display().to_string();
                manager.insert_batch(std::mem::take(&mut batch));
                manager.emit_progress(&app, Some(current_path));
                batch = Vec::with_capacity(PROGRESS_BATCH_SIZE);
            }
        }
    }

    if !batch.is_empty() {
        manager.insert_batch(batch);
    }

    if !manager.should_stop() {
        manager.set_status(super::entry::IndexStatus::Watching, Some(&app));
    }

    manager.emit_progress(&app, None);
}
