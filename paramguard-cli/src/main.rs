pub mod args;
mod cli;
pub mod logic;
pub mod tui;

use crate::cli::{Cli, Commands};
use crate::tui::Tui;
use args::configargs::ConfigSubCommands;
use clap::Parser;
use paramguard_core::logic::env_logic;

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if cli.commands == crate::cli::Commands::Tui {
        match Tui::start() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error starting TUI or the TUI exited unexpectedly: {}", e);
            }
        }
    } else {
        match cli.commands {
            Commands::Config(configs) => match configs.subcommands {
                ConfigSubCommands::Create {
                    name,
                    path,
                    content,
                    env_var,
                } => {
                    if content.is_some() {
                        match env_logic::create_config_file(
                            name,
                            path,
                            if let Some(cont) = content {
                                cont
                            } else {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "Couldn't read the input for the content argument",
                                ));
                            },
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Error creating config file: {}", e);
                            }
                        }
                    } else {
                        match env_logic::create_env_file(name, path, env_var) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Error creating env file: {}", e);
                            }
                        }
                    }
                }
                ConfigSubCommands::Update {
                    name,
                    content,
                    path,
                } => {
                    println!(
                        "Updating config file with name: {}, path: {}, content: {:?}",
                        name, path, content
                    );
                }
                ConfigSubCommands::Archive { name, path } => {
                    println!("Archiving config file with name: {path}/{name}");
                }
            },
            _ => {
                crate::logic::cli_logic::cli_logic_standin();
            }
        }
    }

    Ok(())
}
