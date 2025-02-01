use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, PartialEq)]
pub(crate) struct ConfigArgs {
    #[command(subcommand)]
    pub(crate) subcommands: ConfigCommands,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub(crate) enum ConfigCommands {
    /// Add a configuration file to be tracked
    Add {
        #[arg(
            short,
            long,
            required = true,
            help = "Name of the configuration file to track"
        )]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Path where the configuration file is on disk"
        )]
        path: PathBuf,
    },
    /// Create a new configuration file
    Create {
        #[arg(
            short,
            long,
            required = true,
            help = "Name of the configuration file to create"
        )]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Path to the configuration file to create"
        )]
        path: PathBuf,
        #[arg(short, long, help = "Content of the configuration file to create")]
        content: Option<String>,
        #[arg(
            short,
            long,
            value_delimiter = ' ',
            num_args = 1..,
            conflicts_with = "content",
            help = "Environment variable to use as content of the configuration file to create"
        )]
        env_var: Option<Vec<String>>,
    },
    /// Update a configuration file
    Update {
        #[arg(
            short,
            long,
            required = true,
            help = "Name of the configuration file to update"
        )]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Path to the configuration file to update"
        )]
        path: String,
    },
}
