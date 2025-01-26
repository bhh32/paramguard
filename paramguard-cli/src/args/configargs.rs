use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct ConfigArgs {
    #[structopt(subcommand)]
    pub subcommands: ConfigSubCommands,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum ConfigSubCommands {
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
        path: String,
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
            help = "New content of the configuration file to update"
        )]
        content: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Path to the configuration file to update"
        )]
        path: String,
    },
    /// Archive a configuration file
    Archive {
        #[arg(
            short,
            long,
            required = true,
            help = "Name of the configuration file to archive"
        )]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Path to the configuration file to archive"
        )]
        path: String,
    },
}
