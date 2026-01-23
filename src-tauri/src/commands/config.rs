use crate::fs::{AppError, Config};
use std::path::PathBuf;

fn config_path() -> PathBuf {
    // Use user data directory instead of executable directory for write permissions
    let mut config_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("File Manager");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("config.json")
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
    let json = serde_json::to_string_pretty(&config).map_err(|e| AppError::Io(e.to_string()))?;

    std::fs::write(&path, json).map_err(|e| AppError::Io(e.to_string()))
}
