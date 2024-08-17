use crate::clipboard;

use std::io::{self, Read};

// Copy command
pub fn run() {
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
    clipboard::copy_to_clipboard(text).unwrap();
}
