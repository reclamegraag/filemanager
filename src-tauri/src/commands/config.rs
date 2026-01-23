use crate::fs::{AppError, Config};
use std::path::PathBuf;

fn get_config_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("FileManager")
}

fn config_path() -> PathBuf {
    let config_dir = get_config_dir();
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        log_debug(&format!("Failed to create config dir: {:?}", e));
    }
    config_dir.join("config.json")
}

fn log_debug(msg: &str) {
    let log_path = get_config_dir().join("debug.log");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let line = format!("[{}] {}\n", timestamp, msg);
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
}

#[tauri::command]
pub fn load_config() -> Config {
    let path = config_path();
    log_debug(&format!("Loading from: {:?}", path));

    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                log_debug(&format!("File content length: {} bytes", content.len()));
                match serde_json::from_str::<Config>(&content) {
                    Ok(config) => {
                        log_debug(&format!("Loaded {} bookmarks", config.bookmarks.len()));
                        config
                    }
                    Err(e) => {
                        log_debug(&format!("Parse error: {:?}", e));
                        Config::default()
                    }
                }
            }
            Err(e) => {
                log_debug(&format!("Read error: {:?}", e));
                Config::default()
            }
        }
    } else {
        log_debug("File does not exist, using defaults");
        Config::default()
    }
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), AppError> {
    log_debug("save_config called");
    let path = config_path();
    log_debug(&format!("Saving to: {:?}", path));
    log_debug(&format!("Saving {} bookmarks", config.bookmarks.len()));

    let json = serde_json::to_string_pretty(&config).map_err(|e| {
        log_debug(&format!("Serialize error: {:?}", e));
        AppError::Io(e.to_string())
    })?;

    std::fs::write(&path, &json).map_err(|e| {
        log_debug(&format!("Write error: {:?}", e));
        AppError::Io(e.to_string())
    })?;

    log_debug(&format!("Saved successfully ({} bytes)", json.len()));
    Ok(())
}
