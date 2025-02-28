use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use super::interface::display::TrackedDisplayInfo;
use crate::formatter::{DisplayFormatter, DefaultFormatter, UiType};
use super::types::ConfigFormat;

/// Represents a configuration file managed by ParamGuard.
///
/// Contains metadata about the configuration file including its location,
/// format, content, and last modification time.
#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub name: String,
    pub path: PathBuf,
    pub format: ConfigFormat,
    pub content: String,
    pub last_modified: DateTime<Utc>,
}

impl ConfigFile {
    /// Convert a config file into a tracked file to display its information
    pub fn to_display_info(&self, ui_type: UiType) -> TrackedDisplayInfo {
        let formatter = DefaultFormatter;
        let truncate_lengths = ui_type.get_truncate_lengths();
        let metadata: Option<serde_json::Value> = serde_json::from_str(&self.metadata).ok();
        let size = metadata
            .as_ref()
            .and_then(|md| md["size"].as_u64())
            .map(|size| formatter.format_size(size));
        let tracked = metadata
            .as_ref()
            .and_then(|md| md["tracked"].as_u64())
            .map(|ts| formatter.format_timestamp(ts));
        let modified = metadata
            .as_ref()
            .and_then(|md| md["modified"].as_u64())
            .map(|ts| formatter.format_timestamp(ts));

        TrackedDisplayInfo {
            id: self.id,
            name: formatter.truncate(&self.name, truncate_lengths.as_ref().map(|tl| tl.name)),
            path: formatter.truncate(&self.path, truncate_lengths.as_ref().map(|tl| tl.path)),
            format: self.format.clone(),
            size,
            tracked,
            modified,
            metadata,
        }
    }
}

/// Holds the tracked config file database connection and functionality
pub struct TrackedDb {
    conn: Connection,
}

impl TrackedDb {
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        todo!()
    }

    /// Add a configuration file to the tracked database
    pub fn track_file(
        &self,
        name: &str,
        path: &PathBuf,
        content: &[u8],
        format: &str,
        metadata: &str,
    ) {
        todo!()
    }

    /// Restore the previous version of a tracked file
    pub fn restore_version(&self, id: i64) -> SqliteResult<(ConfigFile, Vec<u8>)> {
        todo!()
    }

    /// Query the tracked file database to list all of the configuration files that are tracked
    pub fn list_tracked(&self) -> SqliteResult<Vec<ConfigFile>> {
        todo!()
    }

    /// Archives a file and then deletes it from the tracked files database
    pub fn delete_config(&self, id: i64) -> SqliteResult<()> {
        todo!()
    }

    // Search the database for a configuration file
    

    // Get the statistical information of the tracked config file database
}

#[derive(Debug)]
pub struct TrackedStatistics {
    pub total_files_tracked: usize,
    pub total_db_size: u64,
}
