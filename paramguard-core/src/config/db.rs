use chrono::{DateTime, Duration, Local, NaiveDate, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use super::{error::ConfigError, interface::display::TrackedDisplayInfo};
use crate::formatter::{DisplayFormatter, DefaultFormatter, UiType};
use super::types::ConfigFormat;

/// Represents a configuration file managed by ParamGuard.
///
/// Contains metadata about the configuration file including its location,
/// format, content, and last modification time.
#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub id: i64,
    pub name: String,
    pub path: PathBuf,
    pub format: String,
    pub content: String,
    pub content_hash: String,
    pub tracked: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub metadata: String,
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
            path: formatter.truncate(&self.path.to_string_lossy().to_string(), truncate_lengths.as_ref().map(|tl| tl.path)),
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
        let conn = Connection::open(db_path)?;

        // Create table if it doesn't exist.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tracked_files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                format TEXT NOT NULL,
                content_hash TEXT NOT NULL,
                file_content BLOB NOT NULL,
                tracked TEXT NOT NULL,
                last_modified TEXT NOT NULL,
                metadata TEXT,
                UNIQUE(id)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    /// Add a configuration file to the tracked database
    pub fn track_file(
        &self,
        name: &str,
        path: &PathBuf,
        content: &[u8],
        format: &str,
        last_modified: &str,
        metadata: &str,
    ) -> SqliteResult<i64> {
        // Calculate content hash
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());

        self.conn.execute(
            "INSERT INTO tracked_files
            (name, path, format, content_hash, file_content, tracked, last_modified, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                name,
                path.to_string_lossy().to_string(),
                format,
                hash,
                content,
                Utc::now().to_string(),
                last_modified,
                metadata
            ]
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Restore the previous version of a tracked file
    pub fn restore_version(&self, id: i64) -> SqliteResult<(ConfigFile)> {
        let mut query = self.conn.prepare(
            "SELECT id, name, path, format, content_hash, file_content, last_modified, metadata
            FROM tracked_files
            WHERE id = ?1"
        )?;

        let mut rows = query.query([id])?;

        if let Some(row) = rows.next()? {
            let tracked_file = ConfigFile {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                format: row.get(3)?,
                content_hash: row.get(4)?,
                content: row.get(5)?,
                tracked: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modified: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
                metadata: row.get(8)?,
            };

            Ok((tracked_file))
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    /// Query the tracked file database to list all of the configuration files that are tracked
    pub fn list_tracked(&self) -> SqliteResult<Vec<ConfigFile>> {
        let mut query = self.conn.prepare(
            "SELECT id, name, path, format, content_hash, tracked, last_modified, metadata
            FROM tracked_files
            ORDER BY tracked DESC"
        )?;

        let tracked_iter = query.query_map([], |row| {
            Ok(ConfigFile {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                format: row.get(3)?,
                content_hash: row.get(4)?,
                content: "".to_string(),
                tracked: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modified: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
                metadata: row.get(8)?,
            })
        })?;

        tracked_iter.collect()
    }

    /// Archives a file and then deletes it from the tracked files database
    pub fn delete_config(&self, id: i64) -> SqliteResult<()> {
        // First get all of the information from the database for the file to be archived
        let mut query = self.conn.prepare(
            "SELECT id, name, path, content, format, metadata
            FROM tracked_files 
            WHERE id = ?1
            ORDER BY id DESC
            LIMIT 1"
        )?;

        let mut rows = query.query([id])?;

        // Archive the latest version
        let archive_db = crate::archive::db::ArchiveDb::new("paramguard.db")?;
        if let Some(row) = rows.next()? {
            match archive_db.archive_file(
                &row.get::<_, String>(1)?,
                &PathBuf::from(row.get::<_, String>(2)?),
                &row.get::<_, Vec<u8>>(3)?,
                &row.get::<_, String>(4)?,
                7,
                "Was deleted while being tracked",
            &row.get::<_, String>(5)?,
            ) {
                Ok(_) => { 
                    println!("File was successfully archived before deletion"); 
                },
                Err(_) => {
                        return Err(
                            rusqlite::Error::QueryReturnedNoRows
                        );
                }
            };
        };

        // Delete the file from the tracked database
        self.conn.execute("DELETE FROM tracked_files WHERE id = ?1", [id])?;

        Ok(())

    }

    // Search the database for a configuration file
    pub fn search_tracked(&self, query: &str) -> SqliteResult<Vec<ConfigFile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, format, content_hash, tracked, last_modified, metadata
            FROM tracked_files
            WHERE name LIKE ?1 OR path LIKE ?1 OR format LIKE ?1
            ORDER BY tracked DESC"
        )?;

        let search_pattern = format!("%{}%", query);
        let tracked_iter = stmt.query_map([search_pattern], |row| {
            Ok(ConfigFile {
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                format: row.get(3)?,
                content_hash: row.get(4)?,
                content: "".to_string(),
                tracked: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modified: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                metadata: row.get(7)?,
            })
        })?;

        tracked_iter.collect()
    }

    // Get the statistical information of the tracked config file database
    pub fn get_statistics(&self) -> SqliteResult<TrackedStatistics> {
        let mut query = self.conn.prepare(
            "SELECT COUNT(*) as total,
            COALESCE(SUM(json_extract(metadata, '$.size')), 0) as total_size,
            AVG(tracked) / 86400.0 as avg_tracked_days
            FROM tracked_files"
        )?;

        query.query_row([], |row| {
            let total: usize = row.get(0)?;
            
            Ok(TrackedStatistics {
                total_files_tracked: total,
                total_db_size: row.get(1)?,
                avg_tracked_days: row.get(2)?,
            })
        })
    }
}

#[derive(Debug)]
pub struct TrackedStatistics {
    pub total_files_tracked: usize,
    pub total_db_size: u64,
    pub avg_tracked_days: f64,
}
