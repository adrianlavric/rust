fn add_space(str: &mut String, n: i32) {
    let mut j = 0;

    while j < n {
        str.push(' ');

        j += 1;
    }
}

fn add_str(str: &mut String, str_to_add: &str) {
    str.push_str(str_to_add);
}

fn add_integer(str: &mut String, n: i32) {
    let mut nr = n;
    let mut nrc = 0;

    let mut str_int = String::from("");

    if nr == 0 {
        str_int.push('0');
    } else {
        if nr < 0 {
            str.push('-');
            nr = -nr;
        }

        while nr > 0 {
            let c = ((nr % 10) as u8 + b'0') as char;
            str_int.push(c);
            nr /= 10;
            nrc += 1;
            if nrc % 3 == 0 && nr > 0 {
                str_int.push('_');
            }
        }

        str_int = str_int.chars().rev().collect();
    }

    str.push_str(&str_int);
}

fn add_float(str: &mut String, n: f64) {
    let mut str_float = String::from("");

    let mut nr = n;

    if nr < 0.0 {
        str.push('-');
        str_float.push('-');
        nr = -nr;
    }

    let p_int = nr as i64;
    let mut p_fr = ((nr - p_int as f64) * 100000000.0).round() as i64;

    let mut nr_cif_0_la_inceput = 0;
    let mut aux = nr;
    aux = aux * 10.0;
    while aux as i64 % 10 == 0 {
        nr_cif_0_la_inceput = nr_cif_0_la_inceput + 1;
        aux = aux * 10.0;
    }

    while p_fr % 10 == 0 {
        p_fr = p_fr / 10;
    }

    let mut str_int = String::from("");
    let mut x = p_int;

    if x == 0 {
        str_int.push('0');
    } else {
        while x > 0 {
            let digit = (x % 10) as u8;
            str_int.push((b'0' + digit) as char);
            x /= 10;
        }
        str_int = str_int.chars().rev().collect();
    }

    str_float.push_str(&str_int);

    let mut str_fr = String::from("");
    let mut y = p_fr;

    if y != 0 {
        str_float.push('.');

        let mut i = 0;
        while i < nr_cif_0_la_inceput {
            str_float.push('0');

            i = i + 1;
        }

        while y > 0 {
            let digit = (y % 10) as u8;
            str_fr.push((b'0' + digit) as char);
            y /= 10;
        }

        str_fr = str_fr.chars().rev().collect();
    }

    str_float.push_str(&str_fr);

    str.push_str(&str_float);
}

fn main() {
    let mut result = String::from("");

    add_space(&mut result, 40);

    add_str(&mut result, "I 💚");

    add_space(&mut result, 1);

    result.push('\n');

    add_space(&mut result, 40);

    add_str(&mut result, "RUST.");

    add_space(&mut result, 1);

    result.push('\n');

    add_space(&mut result, 1);

    result.push('\n');

    add_space(&mut result, 7);

    add_str(&mut result, "Most");

    add_space(&mut result, 12);

    add_str(&mut result, "crate");

    add_space(&mut result, 6);

    add_integer(&mut result, 306437968);

    add_space(&mut result, 11);

    add_str(&mut result, "and");

    add_space(&mut result, 5);

    add_str(&mut result, "lastest");

    add_space(&mut result, 9);

    add_str(&mut result, "is");

    result.push('\n');

    add_space(&mut result, 9);

    add_str(&mut result, "downloaded");

    add_space(&mut result, 8);

    add_str(&mut result, "has");

    add_space(&mut result, 13);

    add_str(&mut result, "downloads");

    add_space(&mut result, 5);

    add_str(&mut result, "the");

    add_space(&mut result, 9);

    add_str(&mut result, "version");

    add_space(&mut result, 4);

    add_float(&mut result, 2.038);

    result.push('.');

    result.push('\n');

    add_space(&mut result, 20);

    result.push('\n');

    println!("{}", result);
}
