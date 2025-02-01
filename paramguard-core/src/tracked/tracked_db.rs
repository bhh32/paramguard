use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

// user super::interface::display::{TrackedDisplayInfo, DefaultFormatter, DisplaFormatter, UiType};
// TODO: Move archive::interface::display.rs to it's own module to be used by other
// parts of the core.

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedFile {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub format: String,
    pub content_hash: String,
    pub tracking_start_date: DateTime<Utc>,
    pub version: u64,
    pub metadata: String,
}

impl TrackedFile {
    //pub fn
}
