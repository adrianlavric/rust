use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Type overflowed!")]
    TypeOverflow,
}

fn checked_add(x: u32, y: u32) -> Result<u32, MyError> {
    let result: u64 = x as u64 + y as u64;
    if result <= std::u32::MAX as u64 {
        Ok(result as u32)
    } else {
        Err(MyError::TypeOverflow)
    }
}

fn checked_multiplication(x: u32, y: u32) -> Result<u32, MyError> {
    let result: u64 = x as u64 * y as u64;
    if result <= std::u32::MAX as u64 {
        Ok(result as u32)
    } else {
        Err(MyError::TypeOverflow)
    }
}

fn main() {
    let result1 = checked_add(100, 200);
    match result1 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let result2 = checked_multiplication(10, 10);
    match result2 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let result3 = checked_add(std::u32::MAX, 999999999);
    match result3 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let result4 = checked_multiplication(std::u32::MAX, 2);
    match result4 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

//[dependencies]
//thiserror = "1.0" in cargo.toml
