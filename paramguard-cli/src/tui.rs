use crate::logic::tui_logic::tui_loop;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Tui;

impl Tui {
    pub fn start() -> Result<(), std::io::Error> {
        println!("Starting TUI...");
        tui_loop()
    }
}
