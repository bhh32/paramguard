use std::io::Write;

pub fn tui_loop() -> Result<(), std::io::Error> {
    loop {
        // Stand-in for actual TUI
        print!("\rTUI to be imemented at a later time. Press Ctrl+C to exit...");

        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
}
