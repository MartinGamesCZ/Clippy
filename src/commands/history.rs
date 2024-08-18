use crate::data_store;

// History command
pub fn run() {
    // Get the history
    let history = data_store::read_history();

    // Print the history
    for (i, item) in history.lines().rev().enumerate() {
        println!("{:2}: {}", i + 1, item);
    }
}
