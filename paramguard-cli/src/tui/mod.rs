pub mod components;
pub mod screens;
pub mod terminal;
pub mod tui_logic;

use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Tui;

impl Tui {
    pub fn start() -> Result<(), std::io::Error> {
        println!("Starting TUI...");

        Ok(())
    }
}
