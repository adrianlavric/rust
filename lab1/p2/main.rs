fn cmmdc(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let aux = y;
        y = x % y;
        x = aux;
    }
    x
}

fn main() {
    for i in 0..=100 {
        for j in 0..=100 {
            if cmmdc(i, j) == 1 {
                println!("({}, {}) sunt coprime", i, j);
            }
        }
    }
}
