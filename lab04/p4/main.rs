use std::fs;

fn main() {
    let file_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";

    if let Ok(contents) = fs::read_to_string(file_path) {
        for line in contents.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    println!("{} => {}", parts[1], parts[0]);
                }
            }
        }
    } else {
        eprintln!("error");
    }
}
