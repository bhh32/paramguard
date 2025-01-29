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
    Store {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long, default_value = "30")]
        retention_days: i64,
        #[arg(short, long)]
        reason: Option<String>,
    },
    Restore {
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    List,
    Search {
        #[arg(short, long)]
        query: String,
    },
    Cleanup,
}
