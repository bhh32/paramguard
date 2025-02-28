use chrono::{DateTime, Utc};

use crate::config::{
    db::{TrackedDb, TrackedStatistics, ConfigFile},
    error::*,
};
use std::path::PathBuf;

/// Configuration File subsystem functions
pub trait ConfigInterface {
    fn track(&self, name: &str, path: &PathBuf) -> Result<i64, ConfigError>;
    fn rollback(&self, id: i64) -> Result<PathBuf, ConfigError>;
    fn list(&self) -> Result<Vec<ConfigFile>, ConfigError>;
    fn search(&self, query: &str) -> Result<Vec<ConfigFile>, ConfigError>;
    fn delete(&self, id: i64) -> Result<(), ConfigError>;
}

/// High-level config file manager service
pub struct ConfigManager {
    db: TrackedDb,
}

impl ConfigManager {
    pub fn new(db_path: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            db: TrackedDb::new(db_path)?,
        })
    }

    /// Get the statistical information of the configuration file tracking database
    pub fn get_statistics(&self) -> Result<TrackedStatistics, ConfigError> {
        self.db.get_statistics().map_err(ConfigError::DbError)
    }
}

impl ConfigInterface for ConfigManager {
    /// Track a new configuration file
    fn track(&self, name: &str, path: &PathBuf) -> Result<i64, ConfigError> {
        let path_str = format!("{}/{name}", path.to_str().unwrap_or_default());
        // Read file content
        let content = std::fs::read(path_str).map_err(|err| ConfigError::ReadError(err))?;
        let last_modified = std::fs::metadata(path)?.modified().map_err(|err| ConfigError::ReadError(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string()
        )))?;
        // Detect the format from path
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");

        // Create metadata
        let metadata = serde_json::json!({
            "size": content.len(),
            "created": std::fs::metadata(path)?.created().map_err(|err| ConfigError::ReadError(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string()
            )))?,
            "modified": last_modified.duration_since(std::time::UNIX_EPOCH).map_err(|err| ConfigError::ReadError(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string()
            )))?.as_secs(),
        })
        .to_string();

        // Store in database
        let id = self.db.track_file(
            name,
            path,
            &content,
            format,
            {
                let modified: DateTime<Utc> = last_modified.into();
                &format!("{modified}")
            },
            &metadata
        )?;

        Ok(id)
    }

    /// Rollback to the last tracked version of the configuration file
    fn rollback(&self, id: i64) -> Result<PathBuf, ConfigError> {
        // Retrieve tracked file and content
        let tracked_file = self.db.restore_version(id).map_err(|err| match err {
            rusqlite::Error::QueryReturnedNoRows => ConfigError::ConfigNotFound(err.to_string()),
            err => ConfigError::DbError(err),
        })?;

        // Create the restoration path
        let rollback_path = PathBuf::from(&tracked_file.path);

        // Write content to file
        std::fs::write(&rollback_path, tracked_file.content);

        Ok(rollback_path)
    }

    /// List all of the tracked configuration files and their info
    fn list(&self) -> Result<Vec<ConfigFile>, ConfigError> {
        self.db.list_tracked().map_err(ConfigError::DbError)
    }

    /// Given a specific query, search the database for config files and their info
    fn search(&self, query: &str) -> Result<Vec<ConfigFile>, ConfigError> {
        self.db
            .search_tracked(query)
            .map_err(ConfigError::DbError)
    }

    /// Delete a configuration file from being tracked
    fn delete(&self, id: i64) -> Result<(), ConfigError> {
        self.db.delete_config(id).map_err(ConfigError::DbError)
    }
}