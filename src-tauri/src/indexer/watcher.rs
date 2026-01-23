use super::entry::IndexEntry;
use super::IndexManager;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::mpsc;

const DEBOUNCE_MS: u64 = 300;

pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
    #[allow(dead_code)]
    stop_tx: Option<mpsc::Sender<()>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            watcher: None,
            stop_tx: None,
        }
    }

    pub fn start(
        &mut self,
        manager: Arc<IndexManager>,
        roots: Vec<PathBuf>,
        app: AppHandle,
    ) -> Result<(), notify::Error> {
        let manager_clone = manager.clone();
        let app_clone = app.clone();

        let (tx, mut rx) = mpsc::channel::<Event>(1000);

        let watcher_tx = tx.clone();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = watcher_tx.blocking_send(event);
                }
            },
            Config::default().with_poll_interval(Duration::from_millis(DEBOUNCE_MS)),
        )?;

        for root in &roots {
            if root.exists() {
                let _ = watcher.watch(root, RecursiveMode::Recursive);
            }
        }

        self.watcher = Some(watcher);

        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        self.stop_tx = Some(stop_tx);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        handle_fs_event(&manager_clone, &app_clone, event).await;
                    }
                    _ = stop_rx.recv() => {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        self.watcher = None;
        self.stop_tx = None;
    }
}

async fn handle_fs_event(manager: &IndexManager, app: &AppHandle, event: Event) {
    use notify::EventKind;

    for path in event.paths {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                if let Ok(metadata) = std::fs::metadata(&path) {
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

                    let size = if metadata.is_file() {
                        Some(metadata.len())
                    } else {
                        None
                    };

                    let entry = IndexEntry::new(
                        file_name,
                        extension,
                        metadata.is_dir(),
                        modified,
                        size,
                    );

                    manager.insert(path, entry);
                }
            }
            EventKind::Remove(_) => {
                manager.remove(&path);
            }
            _ => {}
        }
    }

    manager.emit_progress(app, None);
}
