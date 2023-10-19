fn add_chars_n(str: &mut String, ch: char, n: i32) {
    
    let mut j = 0;

    while j < n {
        str.push(ch);
        j += 1;
    }
    
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
