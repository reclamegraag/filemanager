use crate::fs::{AppError, FileEntry};
use crate::indexer::entry::{IndexProgress, IndexStatus};
use crate::indexer::{cache, scanner, watcher::FileWatcher, IndexManager};
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, State};

pub struct IndexerState {
    pub manager: Arc<IndexManager>,
    pub watcher: Mutex<FileWatcher>,
}

impl IndexerState {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(IndexManager::new()),
            watcher: Mutex::new(FileWatcher::new()),
        }
    }
}

impl Default for IndexerState {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn start_indexing(
    roots: Vec<String>,
    state: State<'_, IndexerState>,
    app: AppHandle,
) -> Result<(), AppError> {
    let root_paths: Vec<PathBuf> = roots.into_iter().map(PathBuf::from).collect();

    state.watcher.lock().stop();

    let manager = state.manager.clone();
    let app_clone = app.clone();
    let roots_clone = root_paths.clone();

    if cache::load_cache(&manager).unwrap_or(false) && manager.get_count() > 0 {
        manager.set_status(IndexStatus::Watching, Some(&app));
        manager.emit_progress(&app, None);

        let _ = state
            .watcher
            .lock()
            .start(manager.clone(), root_paths, app);
        return Ok(());
    }

    tokio::spawn(async move {
        scanner::scan_directories(manager.clone(), roots_clone.clone(), app_clone.clone()).await;

        let _ = cache::save_cache(&manager);

        let mut watcher = FileWatcher::new();
        let _ = watcher.start(manager, roots_clone, app_clone);
    });

    Ok(())
}

#[tauri::command]
pub async fn search_index(
    query: String,
    limit: Option<usize>,
    state: State<'_, IndexerState>,
) -> Result<Vec<FileEntry>, AppError> {
    let limit = limit.unwrap_or(1000);
    Ok(state.manager.search(&query, limit))
}

#[tauri::command]
pub async fn get_index_status(state: State<'_, IndexerState>) -> Result<IndexProgress, AppError> {
    Ok(IndexProgress {
        status: state.manager.get_status(),
        indexed_count: state.manager.get_count(),
        current_path: None,
    })
}

#[tauri::command]
pub async fn stop_indexing(state: State<'_, IndexerState>, app: AppHandle) -> Result<(), AppError> {
    state.manager.request_stop();
    state.watcher.lock().stop();
    state.manager.set_status(IndexStatus::Idle, Some(&app));
    Ok(())
}

#[tauri::command]
pub async fn clear_index_cache() -> Result<(), AppError> {
    cache::clear_cache().map_err(|e| AppError::Io(e.to_string()))
}
