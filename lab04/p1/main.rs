use std::fs;
use std::io;

fn longest(path: &str) -> Result<(), io::Error> {
    let content = fs::read_to_string(path)?;

    let mut max_byte_length = 0;
    let mut max_char_length = 0;
    let mut longest_byte_line = String::new();
    let mut longest_char_line = String::new();

    for line in content.lines() {
        let byte_length = line.len();
        if byte_length > max_byte_length {
            max_byte_length = byte_length;
            longest_byte_line = line.to_string();
        }

        let char_length = line.chars().count();
        if char_length > max_char_length {
            max_char_length = char_length;
            longest_char_line = line.to_string();
        }
    }

    println!("Longest line by bytes: {}", longest_byte_line);
    println!("Longest line by characters: {}", longest_char_line);

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let path = "src/in.txt";

    match longest(path) {
        Ok(_) => println!("Success!"),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}
