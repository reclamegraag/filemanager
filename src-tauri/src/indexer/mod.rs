pub mod entry;
pub mod scanner;
pub mod watcher;
pub mod cache;

use crate::fs::FileEntry;
use entry::{IndexEntry, IndexStatus, IndexProgress};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

pub struct IndexManager {
    index: Arc<RwLock<HashMap<PathBuf, IndexEntry>>>,
    status: Arc<RwLock<IndexStatus>>,
    roots: Arc<RwLock<Vec<PathBuf>>>,
    stop_signal: Arc<RwLock<bool>>,
}

impl Default for IndexManager {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexManager {
    pub fn new() -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
            status: Arc::new(RwLock::new(IndexStatus::Idle)),
            roots: Arc::new(RwLock::new(Vec::new())),
            stop_signal: Arc::new(RwLock::new(false)),
        }
    }

    pub fn get_status(&self) -> IndexStatus {
        self.status.read().clone()
    }

    pub fn get_count(&self) -> usize {
        self.index.read().len()
    }

    pub fn set_status(&self, status: IndexStatus, app: Option<&AppHandle>) {
        *self.status.write() = status.clone();
        if let Some(app) = app {
            let _ = app.emit("index:status", status);
        }
    }

    pub fn emit_progress(&self, app: &AppHandle, current_path: Option<String>) {
        let progress = IndexProgress {
            status: self.get_status(),
            indexed_count: self.get_count(),
            current_path,
        };
        let _ = app.emit("index:progress", progress);
    }

    pub fn should_stop(&self) -> bool {
        *self.stop_signal.read()
    }

    pub fn request_stop(&self) {
        *self.stop_signal.write() = true;
    }

    pub fn reset_stop(&self) {
        *self.stop_signal.write() = false;
    }

    pub fn clear(&self) {
        self.index.write().clear();
    }

    pub fn insert(&self, path: PathBuf, entry: IndexEntry) {
        self.index.write().insert(path, entry);
    }

    pub fn remove(&self, path: &PathBuf) {
        self.index.write().remove(path);
    }

    pub fn insert_batch(&self, entries: Vec<(PathBuf, IndexEntry)>) {
        let mut index = self.index.write();
        for (path, entry) in entries {
            index.insert(path, entry);
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<FileEntry> {
        let query_lower = query.to_lowercase();
        if query_lower.is_empty() {
            return Vec::new();
        }

        let index = self.index.read();
        let mut results = Vec::with_capacity(limit);

        for (path, entry) in index.iter() {
            let matches = if query_lower.contains('/') || query_lower.contains('\\') {
                path.to_string_lossy().to_lowercase().contains(&query_lower)
            } else {
                entry.name_lower.contains(&query_lower)
            };

            if matches {
                results.push(FileEntry {
                    name: entry.name.clone(),
                    path: path.display().to_string(),
                    extension: entry.extension.clone(),
                    size: entry.size,
                    modified: entry.modified,
                    is_dir: entry.is_dir,
                    is_hidden: entry.name.starts_with('.'),
                    is_symlink: false,
                });

                if results.len() >= limit {
                    break;
                }
            }
        }

        results
    }

    pub fn get_index_clone(&self) -> HashMap<PathBuf, IndexEntry> {
        self.index.read().clone()
    }

    pub fn load_from_cache(&self, data: HashMap<PathBuf, IndexEntry>) {
        *self.index.write() = data;
    }

    pub fn set_roots(&self, roots: Vec<PathBuf>) {
        *self.roots.write() = roots;
    }

    pub fn get_roots(&self) -> Vec<PathBuf> {
        self.roots.read().clone()
    }
}
