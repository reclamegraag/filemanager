use crate::fs::{AppError, Config};
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push("File Manager");
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!("Failed to create config dir: {:?}", e);
    }
    config_dir.join("config.json")
}

#[tauri::command]
pub fn load_config() -> Config {
    let path = config_path();
    println!("[Config] Loading from: {:?}", path);

    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                println!("[Config] File content length: {} bytes", content.len());
                match serde_json::from_str::<Config>(&content) {
                    Ok(config) => {
                        println!("[Config] Loaded {} bookmarks", config.bookmarks.len());
                        for b in &config.bookmarks {
                            println!("[Config] - Bookmark: {} -> {}", b.name, b.path);
                        }
                        config
                    }
                    Err(e) => {
                        eprintln!("[Config] Parse error: {:?}", e);
                        Config::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("[Config] Read error: {:?}", e);
                Config::default()
            }
        }
    } else {
        println!("[Config] File does not exist, using defaults");
        Config::default()
    }
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), AppError> {
    let path = config_path();
    println!("[Config] Saving to: {:?}", path);
    println!("[Config] Saving {} bookmarks", config.bookmarks.len());
    for b in &config.bookmarks {
        println!("[Config] - Bookmark: {} -> {}", b.name, b.path);
    }

    let json = serde_json::to_string_pretty(&config).map_err(|e| {
        eprintln!("[Config] Serialize error: {:?}", e);
        AppError::Io(e.to_string())
    })?;

    std::fs::write(&path, &json).map_err(|e| {
        eprintln!("[Config] Write error: {:?}", e);
        AppError::Io(e.to_string())
    })?;

    println!("[Config] Saved successfully ({} bytes)", json.len());
    Ok(())
}
