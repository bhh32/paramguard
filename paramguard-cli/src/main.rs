pub mod args;
mod cli;
pub mod logic;
pub mod tui;

use crate::cli::Cli;
use crate::tui::Tui;
use clap::Parser;

fn main() -> Result<(), std::io::Error>{
    let cli = Cli::parse();

    if cli.commands == crate::cli::Commands::Tui {
        match Tui::start() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error starting TUI or the TUI exited unexpectedly: {}", e);
            }
        }
    } else {
        println!("Not implemented yet");
    }

    Ok(())
}
