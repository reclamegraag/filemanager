use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name: String,
    pub name_lower: String,
    pub extension: Option<String>,
    pub is_dir: bool,
    pub modified: Option<i64>,
    pub size: Option<u64>,
}

impl IndexEntry {
    pub fn new(
        name: String,
        extension: Option<String>,
        is_dir: bool,
        modified: Option<i64>,
        size: Option<u64>,
    ) -> Self {
        Self {
            name_lower: name.to_lowercase(),
            name,
            extension,
            is_dir,
            modified,
            size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IndexStatus {
    Idle,
    Scanning,
    Watching,
    Error,
}

impl Default for IndexStatus {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexProgress {
    pub status: IndexStatus,
    pub indexed_count: usize,
    pub current_path: Option<String>,
}
