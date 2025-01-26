use crate::args::{archiveargs::ArchiveArgs, configargs::ConfigArgs, envargs::EnvArgs};
use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
#[command(author, about, version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

/// Subcommands that the CLI can accept
#[derive(Clone, Debug, Subcommand, PartialEq)]
pub enum Commands {
    #[clap(name = "env", about = "Manage environment variables", alias = "e")]
    Env(EnvArgs),
    #[clap(name = "config", about = "Manage configuration files", alias = "cfg")]
    Config(ConfigArgs),
    #[clap(about = "Manage archives", alias = "arch")]
    Archive(ArchiveArgs),
    #[clap(about = "Start the TUI", alias = "t")]
    Tui,
}
