#[allow(dead_code)]
mod operations;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub extension: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<i64>,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub is_symlink: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WslDistro {
    pub name: String,
    pub path: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bookmark {
    pub name: String,
    pub path: String,
    pub shortcut: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PaneState {
    pub path: String,
    pub sort_column: String,
    pub sort_ascending: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bookmarks: Vec<Bookmark>,
    pub left_pane: PaneState,
    pub right_pane: PaneState,
    pub window: WindowState,
    pub show_hidden: bool,
    pub recent_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "/".to_string());

        Self {
            bookmarks: vec![
                Bookmark {
                    name: "Home".to_string(),
                    path: home.clone(),
                    shortcut: Some(1),
                },
            ],
            left_pane: PaneState {
                path: home.clone(),
                sort_column: "name".to_string(),
                sort_ascending: true,
            },
            right_pane: PaneState {
                path: home,
                sort_column: "name".to_string(),
                sort_ascending: true,
            },
            window: WindowState {
                x: None,
                y: None,
                width: 1200,
                height: 800,
                maximized: false,
            },
            show_hidden: false,
            recent_paths: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UndoToken {
    pub id: String,
    pub operation: String,
    pub paths: Vec<String>,
    pub backup_paths: Vec<String>,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("Path not found: {0}")]
    NotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
