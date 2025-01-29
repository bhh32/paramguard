use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Archive not found: {0}")]
    NotFound(i64),
    #[error("Retention period not expired")]
    RetentionActive,
}
