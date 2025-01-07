use std::fs;

fn numbers_from_line(line: &str, separator: char) -> (i64, i64) {
    let mut first_split = line.split(",");

    let x: i64 = first_split
        .next()
        .unwrap()
        .split(separator)
        .nth(1)
        .expect("There should be + to split the text")
        .parse()
        .expect("this should be a number");
    let y: i64 = first_split
        .next()
        .unwrap()
        .split(separator)
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

fn calculate_price(
    a_move: (i64, i64),
    b_move: (i64, i64),
    price_coords: (i64, i64),
) -> Option<i64> {
    let (ax, ay) = a_move;
    let (bx, by) = b_move;
    let (x, y) = price_coords;
    if (ax * y - ay * x) % (by * ax - bx * ay) != 0 {
        return None;
    }
    let b = (ax * y - ay * x) / (by * ax - bx * ay);
    if (x - b * bx) % ax != 0 {
        return None;
    }
    let a = (x - b * bx) / ax;

    if 100 >= a && a >= 0 && 100 >= b && b >= 0 {
        return Some(3 * a + b);
    }
    None
}

fn calculate_price2(
    a_move: (i64, i64),
    b_move: (i64, i64),
    price_coords: (i64, i64),
) -> Option<i64> {
    let (ax, ay) = a_move;
    let (bx, by) = b_move;
    let (x, y) = price_coords;
    let x = x + 10000000000000;
    let y = y + 10000000000000;
    if (ax * y - ay * x) % (by * ax - bx * ay) != 0 {
        return None;
    }
    let b = (ax * y - ay * x) / (by * ax - bx * ay);
    if (x - b * bx) % ax != 0 {
        return None;
    }
    let a = (x - b * bx) / ax;

    if a >= 0 && b >= 0 {
        return Some(3 * a + b);
    }
    None
}
//30621
//29142
pub(crate) fn day13(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("File should contain data");
    let mut lines = input.lines().peekable();
    let mut price = 0;
    let mut price2 = 0;
    while let Some(mut line) = lines.next() {
        let a = numbers_from_line(line, '+');
        line = lines.next().unwrap();
        let b = numbers_from_line(line, '+');
        line = lines.next().unwrap();
        let p = numbers_from_line(line, '=');
        lines.next();

        price += calculate_price(a, b, p).unwrap_or(0);
        price2 += calculate_price2(a, b, p).unwrap_or(0);
    }
    println!("Solution for day 13 part 1: {}", price);
    println!("Solution for day 13 part 2: {}", price2);
}
