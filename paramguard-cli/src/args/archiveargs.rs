use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug, PartialEq)]
#[clap(about = "Archive a configuration file")]
pub(crate) struct ArchiveArgs {
    #[command(subcommand)]
    subcommands: Subcommands,
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
pub(crate) enum Subcommands {
    /// Manage settings for archiving configuration files.
    Settings {
        #[arg(
            long,
            help = "Set the retention period for archived files (only matters if auto-remove is set to true)",
            default_value = "30"
        )]
        set_retention_period: u32,
        #[arg(
            long,
            help = "Set if archived configuration files are auto-removed after the retention period.",
            default_value = "true"
        )]
        auto_remove_archived: bool,
    },
}
