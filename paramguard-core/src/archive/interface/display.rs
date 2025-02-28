use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Shows all of the information of an archived configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveDisplayInfo {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub format: String,
    pub age: String,
    pub status: String,
    pub reason: Option<String>,
    pub size: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub retention_remaining: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
