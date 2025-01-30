use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, PartialEq)]
#[clap(about = "Archive a configuration file")]
pub(crate) struct ArchiveArgs {
    #[command(subcommand)]
    subcommands: ArchiveCommands,
    #[arg(
        short,
        long,
        required = true,
        help = "Name of the configuration file to archive"
    )]
    pub(crate) name: String,
    #[arg(
        short,
        long,
        required = true,
        help = "Path to the configuration file to archive"
    )]
    pub(crate) path: String,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub(crate) enum ArchiveCommands {
    /// Store a file in the archive
    Store {
        /// Name for the archive file
        #[arg(short, long)]
        name: String,
        /// Path to the archived file
        #[arg(short, long)]
        path: PathBuf,
        /// Number of days to retain the archive (default: 30)
        #[arg(short, long, default_value = "30")]
        retention_days: i64,
        /// Reason for archiving
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// Restore an archived file
    Restore {
        /// Id of the archive to restore
        #[arg(short, long)]
        id: i64,
        /// Optional output path (defaults to original location)
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    /// List all archived files
    List {
        /// Optional limit to number of entries
        #[arg(short, long)]
        limit: Option<usize>,
        /// Show only expired archives
        #[arg(short, long)]
        expired: bool,
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Search archived files
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,
        /// Show detailed information within the results
        #[arg(short, long)]
        detailed: bool,
    },
    /// Clean up expired archives
    Cleanup {
        /// Dry run (show what would be deleted)
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Show archive statistics
    Stats,
    /// Modify archive retention
    Retention {
        /// ID of the archive
        #[arg(short, long)]
        id: i64,

        /// New retention period in days
        #[arg(short, long)]
        days: i64,
    },
}
