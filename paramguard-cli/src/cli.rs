use crate::args::{archiveargs::ArchiveArgs, configargs::ConfigArgs, envargs::EnvArgs};
use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
#[command(author, about, version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Clone, Debug, Subcommand, PartialEq)]
pub enum Commands {
    #[clap(name = "env", about = "Manage environment variables")]
    Env(EnvArgs),
    #[clap(name = "config", about = "Manage configuration files")]
    Config(ConfigArgs),
    Archive(ArchiveArgs),
    Tui,
}
