pub mod args;
mod cli;
pub mod commands;
pub mod logic;
pub mod tui;

use crate::cli::{Cli, Commands};
use crate::tui::{components::editor, Tui};
use args::configargs::ConfigSubCommands;
use clap::Parser;
use paramguard_core::logic::{config_logic, env_logic};

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
            Commands::Config(configs) => match configs.subcommands {
                ConfigSubCommands::Create {
                    name,
                    path,
                    content,
                    env_var,
                } => {
                    // Even if the flag was set, the content might not be there - so we need to check
                    if content.is_some() {
                        // If the content is there, we create a config file with the content
                        match config_logic::create_config_file(
                            name.clone(),
                            path,
                            // Get the value of content from the Option
                            if let Some(cont) = content {
                                cont
                            } else {
                                // Return an error if getting the content failed
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "Couldn't read the input for the content argument",
                                ));
                            },
                        ) {
                            // If the file was created successfully, we print a success message.
                            Ok(_) => println!("{name} was created successfully!"),
                            // If the file creation failed, we print an error
                            Err(e) => {
                                eprintln!("Error creating config file: {}", e);
                                return Err(e);
                            }
                        }
                    // If the content is not there, we create an env file with the env_var
                    } else {
                        // Create the env file
                        match env_logic::create_env_file(name.clone(), path, env_var) {
                            // If the file was created successfully, we print a success message.
                            Ok(_) => println!("{name} was created successfully!"),
                            // If the file creation failed, we print an error
                            Err(e) => {
                                eprintln!("Error creating env file: {}", e);
                                return Err(e);
                            }
                        }
                    }
                }
                ConfigSubCommands::Update { name, path } => {
                    let file_path = format!("{path}/{name}");
                    match config_logic::update_config_file(name.clone(), path.clone(), |content| {
                        editor::edit_file_content(content, &file_path)
                    }) {
                        Ok(_) => {
                            println!("Successfully updated config file: {path}/{name}");
                        }
                        Err(e) => {
                            eprintln!("Error updating config file: {}", e);
                            return Err(e);
                        }
                    }
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
