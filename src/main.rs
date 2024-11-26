use std::io;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (option, file_path) = match args.len() {
        2 => ("ALL".to_string(), args[1].to_string()),
        3 => (args[1].to_string(), args[2].to_string()),
        _ => {
            eprintln!("Usage <option> <file_path>");
            std::process::exit(1);
        }
    };

    if !["ALL", "-c", "-l", "-w", "-m"].contains(&option.as_str()) {
        eprintln!("Invalid option: {}. Expected one of -c, -l, -w, -m or nothing at all", option);
        std::process::exit(1);
    }

    let mut file: File = File::open(file_path.clone())?;
    let mut buffer: Vec<u8> = Vec::new();
    let mut byte_count: i32 = 0;
    let mut line_count: i32 = 0;
    let mut word_count: i32 = 0;
    let mut is_last_char_empty: bool = true;
    let mut character_count: i32 = 0;

    while let Ok(bytes_read) = file.read_to_end(&mut buffer) {
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
        "ALL" => println!("{} {} {} {}", line_count, word_count, byte_count, file_path),
        "-c" => println!("{}", byte_count),
        "-l" => println!("{}", line_count),
        "-w" => println!("{}", word_count),
        "-m" => println!("{}", character_count),
        _ => panic!("Invalid option: {}", option),
    }

    Ok(())
}
