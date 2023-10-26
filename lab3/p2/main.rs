fn checked_add(x: u32, y: u32) -> u32 {
    let result: u64 = x as u64 + y as u64;
    if result > std::u32::MAX as u64 {
        panic!("Type overflow");
    }
    result as u32
}

fn checked_multiplication(x: u32, y: u32) -> u32 {
    let result: u64 = x as u64 * y as u64;
    if result > std::u32::MAX as u64 {
        panic!("Type overflow");
    }
    result as u32
}

fn main() {
    let result1 = checked_add(100, 200);
    let result2 = checked_multiplication(100, 200);
    let result3 = checked_add(std::u32::MAX, 999999999);
    let result4 = checked_multiplication(std::u32::MAX, 2);

    println!("{}, {}, {}, {}", result1, result2, result3, result4);
}
