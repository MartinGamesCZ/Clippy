use crossclip::{Clipboard, SystemClipboard};
use std::env;
use std::io::{self, IsTerminal, Read};
use std::{thread, time};

// Main function
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are any arguments (other than the binary path)
    if args.len() < 2 {
        // If there are no arguments, run the default command
        cmd_default();
        return;
    }

    // Check the first argument and run the corresponding command
    match args[1].as_str() {
        "copy" => cmd_copy(),
        "paste" => cmd_paste(),
        _ => cmd_default(),
    }
}

// Default command
fn cmd_default() {
    // Check if the output is a terminal
    let is_terminal = io::stdout().is_terminal();

    if is_terminal {
        // If the output is a terminal, run the copy command
        cmd_copy();
    } else {
        // If the output is not a terminal, run the paste command
        cmd_paste();
    }
}

// Copy command
fn cmd_copy() {
    // Get the StdIn and lock it
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    // Variables to store the input text
    let mut line = String::new();
    let mut text = String::new();

    // Read the text from the StdIn
    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        // If there are no more bytes to read, break the loop
        if n_bytes == 0 {
            break;
        }

        // Append the line to the text
        text += &line;

        // Clear the line
        line.clear();
    }

    // Copy the text to the clipboard
    copy_text(text).unwrap();
}

// Paste command
fn cmd_paste() {
    // Get the text from the clipboard
    let text = get_text().unwrap();

    // Print the text to the StdOut
    print!("{}", text);
}

// Function to copy text to the clipboard
fn copy_text(text: String) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new SystemClipboard instance
    let clipboard = SystemClipboard::new()?;

    // Copy the text to the clipboard
    clipboard.set_string_contents(text)?;
    println!("Text copied to clipboard");

    // Sleep for 100 milliseconds - needed for the text to be copied
    // TODO: Find a better way to wait for the text to be copied
    thread::sleep(time::Duration::from_millis(100));

    // Resolve the function with no errors
    return Ok(());
}

// Function to get text from the clipboard
fn get_text() -> Result<String, Box<dyn std::error::Error>> {
    // Create a new SystemClipboard instance
    let clipboard = SystemClipboard::new()?;

    // Get the text from the clipboard
    let text = clipboard.get_string_contents()?;

    // Resolve the function with the text
    return Ok(text);
}
