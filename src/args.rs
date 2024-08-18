use std::env;

// Function for getting command line arguments
pub fn get_args() -> Vec<String> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Return the arguments, excluding the binary path
    return args[1..].to_vec();
}

// Function for parsing flags
pub fn parse_flags(args: Vec<String>) -> (Vec<(String, String)>, Vec<String>) {
    // Initialize the flags and arguments
    let mut flags = Vec::new();
    let mut arguments = Vec::new();

    // Iterate over the arguments and separate the flags from the arguments
    for mut i in 0..args.len() {
        // Get the argument
        let arg = &args[i];

        // Check if the argument is a flag
        if arg.starts_with("-") {
            // Get the flag and its value
            let flag = arg.clone();
            let value = if i + 1 < args.len() && !args[i + 1].starts_with("-") {
                i += 1;
                args[i].clone()
            } else {
                String::new()
            };

            // Add the flag and its value to the flags
            flags.push((flag, value));
        } else {
            // Add the argument to the arguments
            arguments.push(arg.clone());
        }
    }

    // Return the flags and arguments
    return (flags, arguments);
}
