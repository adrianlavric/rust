fn add_chars_n(str:String, ch:char, n:i32) -> String {

    let mut q: String = String::from(str);
    let mut j: i32 = 0;

    while j < n {
        
        q.push(ch);
        j += 1;
        
    }

    q

}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}