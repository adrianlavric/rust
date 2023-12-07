use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    //let file = File::open("C:/Users/adria/OneDrive/Desktop/rust/lab08/p1/src/in.txt")?;
    let file = File::open("in.txt")?;
    let mut word_count = HashMap::new();

    for line in io::BufReader::new(file).lines() {
        for word in line?.to_lowercase().split_whitespace() {
            let cleaned_word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            *word_count.entry(cleaned_word).or_insert(0) += 1;
        }
    }

    let mut sorted_word_count: Vec<_> = word_count.into_iter().collect();
    sorted_word_count.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in sorted_word_count {
        println!("{:<15} => {}", word, count);
    }

    Ok(())
}
