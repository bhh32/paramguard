use serde::{Deserialize, Serialize};

/// Shows all of the information of a tracked configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedDisplayInfo {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub format: String,
    pub size: Option<String>,
    pub tracked: Option<String>,
    pub modified: Option<String>,
    pub metadata: Option<serde_json::Value>,
}