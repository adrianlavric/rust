use std::io;

fn rot13_char(c: char) -> char {
    match c {
        'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
        'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
        _ => c,
    }
}

fn rot13(input: &str) -> String {
    input.chars().map(|c| rot13_char(c)).collect()
}

fn main() {
    println!("Input:");
    let mut input_text = String::new();

    io::stdin().read_line(&mut input_text).expect("error");

    let rot13_text = rot13(&input_text);

    println!("rot13: {}", rot13_text);
}
