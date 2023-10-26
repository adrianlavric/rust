use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum CustomError {
    NotAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotAscii => write!(f, "Character is not ASCII"),
            CustomError::NotDigit => write!(f, "Character is not a digit"),
            CustomError::NotHexDigit => write!(f, "Character is not a base16 digit"),
            CustomError::NotLetter => write!(f, "Character is not a letter"),
            CustomError::NotPrintable => write!(f, "Character is not printable"),
        }
    }
}

fn to_uppercase(c: char) -> Result<char, CustomError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CustomError::NotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, CustomError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CustomError::NotLetter)
    }
}

fn is_printable(c: char) -> bool {
    c.is_ascii_graphic() || c.is_ascii_whitespace()
}

fn print_char(c: char) -> Result<(), CustomError> {
    if is_printable(c) {
        println!("{}", c);
        Ok(())
    } else {
        Err(CustomError::NotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u32, CustomError> {
    if c.is_ascii_digit() {
        Ok(c.to_digit(10).unwrap())
    } else {
        Err(CustomError::NotDigit)
    }
}

fn main() {
    let c = 'd';

    match to_uppercase(c) {
        Ok(result) => println!("Uppercase: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }

    let c = '2';

    match char_to_number(c) {
        Ok(result) => println!("Number: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
