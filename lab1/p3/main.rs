fn main() {
    for bottles in (1..=99).rev() {
        let next_bottles = bottles - 1;

        println!(
            "{} bottle{} of beer on the wall,",
            bottles,
            if bottles > 1 { "s" } else { "" }
        );
        println!(
            "{} bottle{} of beer.",
            bottles,
            if bottles > 1 { "s" } else { "" }
        );
        println!("Take one down, pass it around,");

        if next_bottles > 0 {
            println!(
                "{} bottle{} of beer on the wall.\n",
                next_bottles,
                if next_bottles > 1 { "s" } else { "" }
            );
        } else {
            println!("No more bottles of beer on the wall.\n");
        }
    }

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
