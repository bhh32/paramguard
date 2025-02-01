pub mod args;
mod cli;
pub mod commands;
pub mod display;
pub mod logic;
pub mod tui;

use crate::cli::{Cli, Commands};
use crate::commands::{archive::handle_archive_command, config::handle_config_command};
use crate::tui::Tui;
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if cli.commands == crate::cli::Commands::Tui {
        match Tui::start() {
            Ok(_) => {
                // TODO: Start the TUI part of the application.
            }
            Err(e) => {
                eprintln!("Error starting TUI or the TUI exited unexpectedly: {}", e);
                return Err(e);
            }
        }
    } else {
        match cli.commands {
            Commands::Config(cfg_args) => match handle_config_command(&cfg_args.subcommands) {
                Ok(_) => Ok(()),
                Err(e) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )),
            }?,
            Commands::Env(_) => {}
            Commands::Archive(archive_args) => {
                match handle_archive_command(&archive_args.subcommands) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )),
                }?
            }
            _ => {}
        }
    }

    Ok(())
}
