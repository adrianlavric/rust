fn main() {
    
    for i in (1..=99).rev() {
        println!(
            "{} bottle{} of beer on the wall,",
            i,
            if i != 1 { "s" } else { "" }
        );
        println!("{} bottle{} of beer.", i, if i != 1 { "s" } else { "" });
        println!("Take one down, pass it around,");

        if i > 1 {
            println!(
                "{} bottle{} of beer on the wall.",
                i - 1,
                if i != 2 { "s" } else { "" }
            );
        } else {
            println!("No bottles of beer on the wall.");
        }

        println!();
    }
}
