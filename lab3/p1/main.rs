fn prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i: u32 = n / 2;

    while i >= 2 {
        if n % i == 0 {
            return false;
        }
        i -= 1;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut counter: u32 = x as u32 + 1;
    let maxu16 = std::u16::MAX as u32;
    while counter < maxu16 {
        if prime(counter) {
            break;
        }
        counter += 1;
    }
    if counter < maxu16 {
        return Some(counter as u16);
    } else {
        return None;
    }
}

fn main() {
    let mut counter: u32 = 65000;

    while counter < std::u32::MAX {
        let x = next_prime(counter as u16);

        match x {
            Some(value) => {
                println!("{}", value);
                counter = value as u32;
            }
            None => {
                println!("None");
                break;
            }
        }
    }
}
