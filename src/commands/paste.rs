use crate::clipboard;

// Paste command
pub fn run() {
    // Get the text from the clipboard
    let text = clipboard::get_from_clipboard().unwrap();

    // Print the text to the StdOut
    print!("{}", text);
}
