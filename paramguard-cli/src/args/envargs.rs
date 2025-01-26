use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct EnvArgs {
    #[structopt(subcommand)]
    pub subcommands: EnvSubCommands,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum EnvSubCommands {
    /// Add a new environment variable
    Add {
        #[arg(short, long, required = true, help = "Add a new environment variable")]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "Value of the environment variable"
        )]
        value: String,
    },
    /// Get the value of an environment variable
    Get {
        #[arg(
            short,
            long,
            required = true,
            help = "Get the value of an environment variable"
        )]
        name: String,
    },
    /// Update an environment variable
    Update {
        #[arg(
            short,
            long,
            required = true,
            help = "Update the value of an environment variable"
        )]
        name: String,
        #[arg(
            short,
            long,
            required = true,
            help = "New value of the environment variable"
        )]
        value: String,
    },
}
