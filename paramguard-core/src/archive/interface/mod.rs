pub mod display;

use crate::archive::db::{ArchiveDb, ArchiveStatistics, ArchivedFile, RetentionInfo};
use crate::archive::error::*;
use chrono::{Duration, Utc};
use std::path::PathBuf;

pub trait ArchiveInterface {
    fn store(
        &self,
        name: &str,
        path: &PathBuf,
        retention_days: i64,
        reason: Option<String>,
    ) -> Result<i64, ArchiveError>;
    fn restore(&self, id: i64, output_path: Option<PathBuf>) -> Result<PathBuf, ArchiveError>;
    fn list(&self) -> Result<Vec<ArchivedFile>, ArchiveError>;
    fn search(&self, query: &str) -> Result<Vec<ArchivedFile>, ArchiveError>;
    fn cleanup(&self) -> Result<usize, ArchiveError>;
    fn can_delete(&self, id: i64) -> Result<bool, ArchiveError>;
}

// High-level archive operations service
pub struct ArchiveService {
    db: ArchiveDb,
}

impl ArchiveService {
    pub fn new(db_path: &str) -> Result<Self, ArchiveError> {
        Ok(Self {
            db: ArchiveDb::new(db_path)?,
        })
    }

    pub fn delete(&self, id: i64) -> Result<(), ArchiveError> {
        if self.can_delete(id)? {
            self.db.delete_archive(id).map_err(ArchiveError::DbError)
        } else {
            Err(ArchiveError::RetentionActive)
        }
    }

    pub fn get_retention_info(&self, id: i64) -> Result<RetentionInfo, ArchiveError> {
        let archive = self.db.get_archive_info(id).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ArchiveError::NotFound(id),
            e => ArchiveError::DbError(e),
        })?;

        let now = Utc::now();
        let archive_date = archive.archive_date;
        let retention_period = Duration::seconds(archive.retention_period);
        let time_remaining = if now < archive_date + retention_period {
            Some(archive_date + retention_period - now)
        } else {
            None
        };

        Ok(RetentionInfo {
            archive_date,
            retention_period,
            time_remaining,
            can_delete: time_remaining.is_none(),
        })
    }

    pub fn update_retention(&self, id: i64, new_retention_days: i64) -> Result<(), ArchiveError> {
        self.db
            .update_retention_period(id, new_retention_days * 86400)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => ArchiveError::NotFound(id),
                e => ArchiveError::DbError(e),
            })
    }

    pub fn get_statistics(&self) -> Result<ArchiveStatistics, ArchiveError> {
        self.db.get_statistics().map_err(ArchiveError::DbError)
    }
}

impl ArchiveInterface for ArchiveService {
    fn store(
        &self,
        name: &str,
        path: &PathBuf,
        retention_days: i64,
        reason: Option<String>,
    ) -> Result<i64, ArchiveError> {
        // Read file content
        let content = std::fs::read(path).map_err(|err| ArchiveError::IoError(err))?;

        // Detect format from path
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");

        // Create metadata
        let metadata = serde_json::json!({
            "size": content.len(),
            "created": std::fs::metadata(path)?.created().map_err(|e| ArchiveError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string()
            )))?.duration_since(std::time::UNIX_EPOCH).map_err(|e| ArchiveError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string()
            )))?.as_secs(),
            "modified": std::fs::metadata(path)?.modified().map_err(|e| ArchiveError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string()
            )))?.duration_since(std::time::UNIX_EPOCH).map_err(|e| ArchiveError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string()
            )))?.as_secs(),
        })
        .to_string();

        // Store in database
        let id = self.db.archive_file(
            name,
            path,
            &content,
            format,
            retention_days,
            &reason.unwrap_or_else(|| "No reason provided".to_string()),
            &metadata,
        )?;

        Ok(id)
    }

    fn restore(&self, id: i64, output_path: Option<PathBuf>) -> Result<PathBuf, ArchiveError> {
        // Retrieve archived file and content
        let (archived_file, content) = self.db.restore_file(id).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ArchiveError::NotFound(id),
            e => ArchiveError::DbError(e),
        })?;

        // Determine restore path
        let restore_path = if let Some(output_path) = output_path {
            if output_path.is_dir() {
                // If output_path is a directory, use original filename
                output_path.join(PathBuf::from(&archived_file.name))
            } else {
                output_path
            }
        } else {
            // Use original path if no output path specified
            PathBuf::from(&archived_file.original_path)
        };

        // Ensure parent directory exists
        if let Some(parent) = restore_path.parent() {
            std::fs::create_dir_all(parent).map_err(ArchiveError::IoError)?;
        }

        // Write content to file
        std::fs::write(&restore_path, content).map_err(ArchiveError::IoError)?;

        Ok(restore_path)
    }

    fn list(&self) -> Result<Vec<ArchivedFile>, ArchiveError> {
        self.db.list_archives().map_err(ArchiveError::DbError)
    }

    fn search(&self, query: &str) -> Result<Vec<ArchivedFile>, ArchiveError> {
        self.db
            .search_archives(query)
            .map_err(ArchiveError::DbError)
    }

    fn cleanup(&self) -> Result<usize, ArchiveError> {
        self.db.cleanup_expired().map_err(ArchiveError::DbError)
    }

    fn can_delete(&self, id: i64) -> Result<bool, ArchiveError> {
        self.db.can_delete(id).map_err(ArchiveError::DbError)
    }
}
