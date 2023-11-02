use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";

    let file_contents = fs::read_to_string(file_path)?;

    let lines = file_contents.lines();

    for line in lines {
        let replaced_line = replace_abbreviations(line);
        println!("{}", replaced_line);
    }

    Ok(())
}

fn replace_abbreviations(line: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();

    for c in line.chars() {
        if c.is_whitespace() {
            result.push_str(&replace_word(&word));
            result.push(c);
            word.clear();
        } else {
            word.push(c);
        }
    }

    result.push_str(&replace_word(&word));
    result
}

fn replace_word(word: &str) -> &str {
    match word {
        "pt" => "pentru",
        "ptr" => "pentru",
        "dl" => "domnul",
        "dna" => "doamna",
        _ => word,
    }
}
