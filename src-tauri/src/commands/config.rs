use crate::fs::{AppError, Config};
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    exe_dir.join("config.json")
}

#[tauri::command]
pub fn load_config() -> Config {
    let path = config_path();

    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Config::default()
    }
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), AppError> {
    let path = config_path();
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| AppError::Io(e.to_string()))?;

    std::fs::write(&path, json)
        .map_err(|e| AppError::Io(e.to_string()))
}
