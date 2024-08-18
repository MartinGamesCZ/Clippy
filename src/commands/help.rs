// Help command
pub fn run() {
    // Define the commands
    let commands = vec![
        ("copy", "Copy text to the clipboard"),
        ("paste", "Paste text from the clipboard"),
        ("history", "Show the history"),
        ("help", "Show this help message"),
    ];

    // Define the flags
    let flags = vec![(
        "-h, --history",
        "Get nth last entry from the history, can be used when pasting.",
    )];

    // Print the help message
    println!("Usage: clippy [command] (flags)");
    println!();
    println!("Commands:");
    for (name, description) in commands {
        println!("  {:<10} {}", name, description);
    }

    println!();
    println!("Flags:");
    for (name, description) in flags {
        println!("  {:<15} {}", name, description);
    }

    println!();
    println!("Usage with pipes:");
    println!("  echo 'Hello, World!' | clippy copy");
    println!("  clippy paste > file.txt");

    println!();
    println!("Tip: If no command is provided, it will be determined.");
}
