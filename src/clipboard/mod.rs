use crossclip::{Clipboard, SystemClipboard};
use std::{error::Error, thread, time};

// Function to copy text to the clipboard
pub fn copy_to_clipboard(text: String) -> Result<(), Box<dyn Error>> {
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
pub fn get_from_clipboard() -> Result<String, Box<dyn Error>> {
    // Create a new SystemClipboard instance
    let clipboard = SystemClipboard::new()?;

    // Get the text from the clipboard
    let text = clipboard.get_string_contents()?;

    // Resolve the function with the text
    return Ok(text);
}
