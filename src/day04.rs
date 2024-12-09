use std::fs;

static LETTERS: [u8; 3] = [b'M', b'A', b'S']; // 77,65,83
static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn find_xmas(
    input: &Vec<&str>,
    x: i32,
    y: i32,
    rows: i32,
    cols: i32,
    dx: i32,
    dy: i32,
    step: usize,
) -> i32 {
    if step == 3 {
        return 1;
    }
    if 0 > y || y >= rows || 0 > x || x >= cols {
        return 0;
    }
    let letter = input[y as usize].bytes().nth(x as usize).unwrap();
    if letter == LETTERS[step] {
        return find_xmas(input, x + dx, y + dy, rows, cols, dx, dy, step + 1);
    }
    return 0;
}

fn find_x_mas(input: &Vec<&str>, x: usize, y: usize, cols: usize, rows: usize) -> bool {
    let left = x.checked_sub(1);
    let right = x + 1;
    let bottom = y + 1;
    let top = y.checked_sub(1);
    if left == None || top == None || bottom >= rows || right >= cols {
        return false;
    }
    let left = left.unwrap();
    let top = top.unwrap();
    let tl = input[top].bytes().nth(left).unwrap();
    let bl = input[bottom].bytes().nth(left).unwrap();
    let tr = input[top].bytes().nth(right).unwrap();
    let br = input[bottom].bytes().nth(right).unwrap();
    (tl == b'M' && bl == b'M' && tr == b'S' && br == b'S')
        || (tl == b'S' && bl == b'S' && tr == b'M' && br == b'M')
        || (br == b'M' && bl == b'M' && tr == b'S' && tl == b'S')
        || (br == b'S' && bl == b'S' && tr == b'M' && tl == b'M')
}
pub(crate) fn day4(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read input file");
    let mut xmas_count = 0;
    let mut x_mas_count = 0;
    let input_2d: Vec<&str> = input.lines().collect();
    for (y, line) in input_2d.iter().enumerate() {
        for (x, letter) in line.bytes().enumerate() {
            if letter == b'X' {
                for (dx, dy) in DIRECTIONS {
                    xmas_count += find_xmas(
                        &input_2d,
                        x as i32 + dx,
                        y as i32 + dy,
                        input_2d.len() as i32,
                        input_2d[0].len() as i32,
                        dx,
                        dy,
                        0,
                    );
                }
            }
            if letter == b'A' && find_x_mas(&input_2d, x, y, input_2d.len(), input_2d[0].len()) {
                x_mas_count += 1;
            }
        }
    }
    println!("Solution for day 4 part 1: {xmas_count}");
    println!("Solution for day 4 part 2: {x_mas_count}");
}
