mod clipboard;
mod commands;

use std::env;

// Main function
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are any arguments (other than the binary path)
    if args.len() < 2 {
        // If there are no arguments, run the default command
        commands::default::run();
        return;
    }

    // Check the first argument and run the corresponding command
    match args[1].as_str() {
        "copy" => commands::copy::run(),
        "paste" => commands::paste::run(),
        _ => commands::default::run(),
    }
}
