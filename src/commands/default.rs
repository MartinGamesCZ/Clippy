use crate::commands;
use std::io;

// Default command
pub fn run() {
    // Check if the output is a terminal
    let is_terminal = io::stdout().is_terminal();

    if is_terminal {
        // If the output is a terminal, run the copy command
        commands::copy::run();
    } else {
        // If the output is not a terminal, run the paste command
        commands::paste::run();
    }
}
