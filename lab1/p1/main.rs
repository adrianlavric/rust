fn prim(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    } else {
        let mut j = 3;
        while j * j <= n {
            if n % j == 0 {
                return false;
            }
            j += 2;
        }
    }
    return true;
}
fn main() {
    for i in 0..=100 {
        if prim(i) {
            println!("{} e prim", i);
        }
    }
}
