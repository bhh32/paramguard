use crate::archive::error::*;
use crate::archive::{ArchiveDb, ArchivedFile};
use std::path::PathBuf;
use chronos::{DateTime, Duration, Utc};


#[derive(Debug)]
pub struct RetentionInfo {
    pub archive_date: DateTime<Utc>,
    pub retention_period: Duration,
    pub time_remaining: Option<Duration>,
    pub can_delete: bool,
}

#[derive(Debug)]
pub struct ArchiveStatistics {
    pub total_archives: usize,
    pub total_size: u64,
    pub expired_count: usize,
    pub active_count: usize,
    pub avg_retention_days: f64,
}

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
            Err(ArchiveError::RentionActive)
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
        self.db.update_retention_period(id, new_retention_days * 86400)
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
        reason: Option<String>
    ) -> Result<i64, ArchiveError> {
        // Read file content
        let content = std::fs::read(path)
            .map_err(|err| ArchiveError::IoError(e))?;

        // Detect format from path
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");

        // Create metadata
        let metadata = serde_json::json!({
            "size": content.len(),
            "created": std::fs::metadata(path)?
                .created()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            "modified": std::fs::metadata(path)?
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        }).to_string();

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
        let (archived_file, content) = self.db.restore_file(id)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => ArchiveError::NotFound(id),
                e => ArchiveError::DbError(e),
            })?

        // Determine restore path
        let restore_path = if let Some(output_path) = output_path {
            if output_path.is_dir() {
                // If output_path is a directory, use original filename
                output_path.join(PathBuf::from(&archived_file.name))
            } esle {
                output_path
            }
        } else {
            // Use original path if no output path specified
            PathBuf::from(&archived_file.original_path)
        };

        // Ensure parent directory exists
        if let Some(parent) = restore_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(ArchiveError::IoError)?;
        }

        // Write content to file
        std::fs::write(&restore_path, content)
            .map_err(ArchiveError::IoError)?;

        Ok(restore_path)
    }

    fn list(&self) -> Result<Vec<ArchivedFile>, ArchiveError> {
        self.db.list_archives().map_err(ArchiveError::DbError)
    }

    fn search(&self, query: &str) -> Result<Vec<ArchivedFile>, ArchiveError> {
        self.db.search_archives(query).map_err(ArchiveError::DbError)
    }

    fn cleanup(&self) -> Result<usize, ArchiveError> {
        self.db.cleanup_expired().map_err(ArchiveError::DbError)
    }

    fn can_delete(&self, id: i64) -> Result<bool, ArchiveError> {
        self.db.can_delete(id).map_err(ArchiveError::DbError)
    }
}
