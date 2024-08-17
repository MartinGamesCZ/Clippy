use std::env;

// Function for getting command line arguments
pub fn get_args() -> Vec<String> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Return the arguments, excluding the binary path
    return args[1..].to_vec();
}
