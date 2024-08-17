use crossclip::{Clipboard, SystemClipboard};
use std::env;
use std::io::{self, IsTerminal, Read};
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cmd_default();
        return;
    }

    match args[1].as_str() {
        "copy" => cmd_copy(),
        "paste" => cmd_paste(),
        _ => cmd_default(),
    }
}

fn cmd_default() {
    let is_terminal = io::stdout().is_terminal();

    if is_terminal {
        cmd_copy();
    } else {
        cmd_paste();
    }
}

fn cmd_copy() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    let mut text = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }

        text += &line;

        line.clear();
    }

    copy_text(text).unwrap();
}

fn cmd_paste() {
    let text = get_text().unwrap();

    print!("{}", text);
}

fn copy_text(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let clipboard = SystemClipboard::new()?;

    clipboard.set_string_contents(text)?;

    println!("Text copied to clipboard");

    thread::sleep(time::Duration::from_millis(100));

    return Ok(());
}

fn get_text() -> Result<String, Box<dyn std::error::Error>> {
    let clipboard = SystemClipboard::new()?;

    let text = clipboard.get_string_contents()?;

    return Ok(text);
}
