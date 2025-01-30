pub mod db;
pub mod error;
pub mod interface;

// Re-export commonly used types
pub use db::{ArchiveDb, ArchivedFile};
pub use error::ArchiveError;
pub use interface::{ArchiveInterface, ArchiveService};
