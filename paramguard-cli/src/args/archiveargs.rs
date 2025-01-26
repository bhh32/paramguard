use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone, Debug, PartialEq)]
#[clap(about = "Archive a configuration file")]
pub(crate) struct ArchiveArgs {
    #[subcommand]
    subcommands: Subcommands,
    #[arg(short, long, conflicts_with_subcommand = "settings", requires = "path", help = "Name of the configuration file to archive."]
    pub name: String,
    #[arg(short, long, conflicts_with_subcommand = "settings", requires = "name", help = "Path to the configuration file to archive."]
    pub path: String,
}

#[derive(Subcommand)]
pub(crate) enum Subcommands {
    /// Manage settings for archiving configuration files.
    Settings {
        #[arg(long, help = "Set the retention period for archived files (only matters if auto-remove is set to true)", default_val = 30)]
        set_retention_period: u32,
        #[arg(long, help = "Set if archived configuration files are auto-removed after the retention period.", default_value = true]
        auto_remove_archived: bool,
    },
}
