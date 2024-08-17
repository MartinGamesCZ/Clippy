use crate::commands;
use std::io::{self, IsTerminal};

// Default command
pub fn run() {
    // Check if the output is a terminal
    let stdout_is_terminal = io::stdout().is_terminal();
    let stdin_is_terminal = io::stdin().is_terminal();

    // If StdOut is not a terminal, run the paste command
    if !stdout_is_terminal {
        commands::paste::run();
        return;
    }

    // If StdIn is not a terminal, run the copy command
    if !stdin_is_terminal {
        commands::copy::run();
        return;
    }

    // If both StdIn and StdOut are terminals, print information
    println!("Clippy // Linux Clipboard Manager");
    println!("Created by Martin Petr");
    println!();
    println!("-> Warning: Could not determine the command, if you meant to copy/paste, you messed it up.");
    println!();
    println!("Running `clippy help` for you! Here you go:");
    println!();

    commands::help::run();
}
