use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

// Function for initializing the data store
pub fn init() {
    // Get the root directory
    let root_dir = get_homedir() + "/.clippy";

    // Check if the root directory exists
    if Path::new(&root_dir).exists() {
        // If the root directory exists, return
        return;
    }

    // If the root directory does not exist, create it
    fs::create_dir_all(&root_dir).unwrap();

    // Create the history file
    let _ = File::create(root_dir + "/history.dat").unwrap();
}

// Function for writing to the history file
pub fn read_history() -> String {
    // Get the root directory and the history file paths
    let root_dir = get_homedir() + "/.clippy";
    let history_file = root_dir + "/history.dat";

    // Open the history file
    let mut file = File::open(history_file).unwrap();

    // Read the contents of the history file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Return the contents of the history file
    return contents;
}

// Function for writing to the history file
pub fn write_history(text: String) {
    // Get the root directory and the history file paths
    let root_dir = get_homedir() + "/.clippy";
    let history_file = root_dir + "/history.dat";

    // Open the history file
    let mut file = File::open(&history_file).unwrap();

    // Read the contents of the history file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Append the new text to the contents
    contents += &text;
    contents += "\n";

    // Remove old entries (keep only the last 25 lines)
    let lines: Vec<&str> = contents.lines().collect();
    let start = if lines.len() > 26 {
        lines.len() - 26
    } else {
        0
    };
    contents = lines[start..].join("\n");

    // Write the new contents to the history file
    let mut file = File::create(history_file).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

// Function for getting the home directory
fn get_homedir() -> String {
    let home = std::env::var("HOME").unwrap();
    return home;
}
