use args::get_args;

mod args;
mod clipboard;
mod commands;
mod data_store;

// Main function
fn main() {
    // Initialize the data store
    data_store::init();

    // Get the command line arguments
    let args = get_args();

    // Check if there are any arguments
    if args.len() < 1 {
        // If there are no arguments, run the default command
        commands::default::run();
        return;
    }

    // Check the first argument and run the corresponding command
    match args[0].as_str() {
        "copy" => commands::copy::run(),
        "paste" => commands::paste::run(),
        "help" => commands::help::run(),
        "history" => commands::history::run(),
        _ => commands::default::run(),
    }
}
