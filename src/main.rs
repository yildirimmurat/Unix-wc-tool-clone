use std::io;
use std::env;
use std::fs::File;
use std::fs::metadata;
use std::io::Read;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Ensure we have at least one argument
    if args.len() < 2 {
        eprintln!("Usage: ccwc [-c|-l|-w|-m] [file_path]");
        std::process::exit(1);
    }

    // Initialize option and input source
    let (option, input_source) = if args.len() == 2 {
        // If only one argument is provided, treat it as a file path or option
        if let Ok(_) = metadata(&args[1]) {
            // If it's a valid file path, assume it's the file path and use "ALL"
            ("ALL".to_string(), args[1].clone())
        } else {
            // Otherwise, treat it as an option with stdin as the source
            (args[1].clone(), "".to_string())
        }
    } else if args.len() == 3 {
        // If two arguments are provided, the first is the option and the second is the file path
        (args[1].clone(), args[2].clone())
    } else {
        eprintln!("Usage: ccwc [-c|-l|-w|-m] [file_path]");
        std::process::exit(1);
    };

    if !["ALL", "-c", "-l", "-w", "-m"].contains(&option.as_str()) {
        eprintln!("Invalid option: {}. Expected one of -c, -l, -w, -m or nothing at all", option);
        std::process::exit(1);
    }

    // Choose the input source (stdin or file)
    let mut input: Box<dyn Read> = if input_source == "" {
        // Read from stdin if the input source is "-"
        Box::new(io::stdin())
    } else {
        // Otherwise, try to open the specified file
        Box::new(File::open(&input_source)?)
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut byte_count: i32 = 0;
    let mut line_count: i32 = 0;
    let mut word_count: i32 = 0;
    let mut is_last_char_empty: bool = true;
    let mut character_count: i32 = 0;

    while let Ok(bytes_read) = input.read_to_end(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        byte_count += bytes_read as i32;
        for &byte in &buffer {
            if byte == b'\n' { // for windows '\r\n -> https://stackoverflow.com/questions/47541191/how-to-get-current-platform-end-of-line-character-sequence-in-rust
                line_count += 1;
            }

            if byte.is_ascii_whitespace() && !is_last_char_empty {
                word_count += 1;
                is_last_char_empty = true;
            }

            if !byte.is_ascii_whitespace() && is_last_char_empty {
                is_last_char_empty = false;
            }
        }

        let content = String::from_utf8_lossy(&buffer);
        character_count += content.chars().count() as i32;

        buffer.clear();
    }

    match option.as_str() {
        "ALL" => println!("{} {} {} {}", line_count, word_count, byte_count, input_source),
        "-c" => println!("{}", byte_count),
        "-l" => println!("{}", line_count),
        "-w" => println!("{}", word_count),
        "-m" => println!("{}", character_count),
        _ => panic!("Invalid option: {}", option),
    }

    Ok(())
}
