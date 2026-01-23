use super::entry::IndexEntry;
use super::IndexManager;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const CACHE_FILE_NAME: &str = "file_index_cache.json";
const CACHE_MAX_AGE_HOURS: u64 = 24;

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheData {
    timestamp: u64,
    entries: HashMap<PathBuf, IndexEntry>,
}

pub fn get_cache_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("filemanager").join(CACHE_FILE_NAME))
}

pub fn save_cache(manager: &IndexManager) -> io::Result<()> {
    let cache_path = match get_cache_path() {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No cache directory",
            ))
        }
    };

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let data = CacheData {
        timestamp,
        entries: manager.get_index_clone(),
    };

    let file = fs::File::create(&cache_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

pub fn load_cache(manager: &IndexManager) -> io::Result<bool> {
    let cache_path = match get_cache_path() {
        Some(p) => p,
        None => return Ok(false),
    };

    if !cache_path.exists() {
        return Ok(false);
    }

    let file = fs::File::open(&cache_path)?;
    let reader = BufReader::new(file);
    let data: CacheData = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let age_secs = now.saturating_sub(data.timestamp);
    let max_age_secs = Duration::from_secs(CACHE_MAX_AGE_HOURS * 3600).as_secs();

    if age_secs > max_age_secs {
        return Ok(false);
    }

    manager.load_from_cache(data.entries);
    Ok(true)
}

pub fn clear_cache() -> io::Result<()> {
    if let Some(cache_path) = get_cache_path() {
        if cache_path.exists() {
            fs::remove_file(&cache_path)?;
        }
    }
    Ok(())
}
