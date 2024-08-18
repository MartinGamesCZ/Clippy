use crate::{args, clipboard, data_store};

// Paste command
pub fn run() {
    // Get flags
    let flags = args::parse_flags(args::get_args()).0;

    // Get the history flag
    let history = flags
        .iter()
        .find(|(flag, _)| flag == "-h" || flag == "--history");

    // Check if the history flag is present
    if let Some((_, value)) = history {
        // Get the history
        let text = data_store::read_history();

        // Get the nth last entry from the history
        let nth_last = if value.is_empty() {
            0
        } else {
            value.parse::<usize>().unwrap()
        };

        // Check if value for flag is present
        if nth_last <= 0 {
            // Print an error message
            println!("Error: Value for history flag must be a positive integer between 1 and 25.");
            return;
        }

        // Get the nth last entry from the history
        let text = text.lines().rev().nth(nth_last - 1).unwrap_or("");

        // Print the text to the StdOut
        print!("{}", text);
        return;
    }

    // Get the text from the clipboard
    let text = clipboard::get_from_clipboard().unwrap();

    // Print the text to the StdOut
    print!("{}", text);
}
