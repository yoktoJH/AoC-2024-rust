use std::fs;

fn translate_instruction(c: u8) -> (i32, i32) {
    match c {
        b'<' => (-1, 0),
        b'^' => (0, -1),
        b'>' => (1, 0),
        b'v' => (0, 1),
        _ => (-69, -420),
    }
}
fn push(map: &mut Vec<Vec<u8>>, (x_shift, y_shift): (i32, i32), (x0, y0): (i32, i32)) -> bool {
    if map[y0 as usize][x0 as usize] == b'#' {
        return false;
    }
    if map[y0 as usize][x0 as usize] == b'.' {
        return true;
    }
    let x = x0 + x_shift;
    let y = y0 + y_shift;
    if push(map, (x_shift, y_shift), (x, y)) {
        map[y as usize][x as usize] = map[y0 as usize][x0 as usize];
        map[y0 as usize][x0 as usize] = b'.';
        return true;
    }
    false
}
fn count_score(map: &Vec<Vec<u8>>, scored_character: u8) -> usize {
    let mut score = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == scored_character {
                score += 100 * i + j;
            }
        }
    }
    score
}
fn move_robot(
    map: &mut Vec<Vec<u8>>,
    (x_shift, y_shift): (i32, i32),
    (x0, y0): (i32, i32),
) -> (i32, i32) {
    let x = x0 + x_shift;
    let y = y0 + y_shift;
    //map[y as usize][x as usize] == b'.' ||
    if  push(map, (x_shift, y_shift), (x, y)) {
        return (x, y);
    }
    (x0, y0)
}

fn push_2x(
    map: &mut Vec<Vec<u8>>,
    (x_shift, y_shift): (i32, i32),
    (x0, y0): (i32, i32),
    just_checking: bool,
) -> bool {
    if y_shift == 0 {
        return push(map, (x_shift, y_shift), (x0, y0));
    }
    if map[y0 as usize][x0 as usize] == b'#' {
        return false;
    }
    if map[y0 as usize][x0 as usize] == b'.' {
        return true;
    }

    let mut xleft = 0;
    let mut xright = 0;
    let y = y0+ y_shift;
    if map[y0 as usize][x0 as usize] == b'[' {
        xleft = x0 + x_shift;
        xright = x0 + 1+ x_shift;
    } else {
        xleft = x0 - 1+ x_shift;
        xright = x0+ x_shift;
    }
    if !push_2x(map,(x_shift,y_shift),(xleft,y),just_checking)
    || !push_2x(map,(x_shift,y_shift),(xright,y),just_checking) {
        return false;
    }

    if !just_checking {
        map[y as usize][xleft as usize] = b'[';
        map[y0 as usize][(xleft - x_shift) as usize] = b'.';

        map[y as usize][xright as usize] = b']';
        map[y0 as usize][(xright - x_shift) as usize] = b'.';
    }
    true
}
fn move_robot_2x(
    map: &mut Vec<Vec<u8>>,
    (x_shift, y_shift): (i32, i32),
    (x0, y0): (i32, i32),
) -> (i32, i32) {
    let x = x0 + x_shift;
    let y = y0 + y_shift;
    /*if map[y as usize][x as usize] == b'.' {
        return (x, y);
    }*/
    if  push_2x(map, (x_shift, y_shift), (x, y),true) {
        push_2x(map, (x_shift, y_shift), (x, y),false);
        return (x,y);
    }
    (x0, y0)
}
fn create_2x_map(map: &Vec<Vec<u8>>, x: &mut i32) -> Vec<Vec<u8>> {
    let mut map_2x = Vec::new();

    for v in map {
        let mut vec = Vec::new();
        for c in v {
            match c {
                b'.' => {
                    vec.push(b'.');
                    vec.push(b'.');
                }
                b'@' => {
                    *x = vec.len() as i32;
                    vec.push(b'@');
                    vec.push(b'.');
                }
                b'#' => {
                    vec.push(b'#');
                    vec.push(b'#');
                }
                b'O' => {
                    vec.push(b'[');
                    vec.push(b']');
                }
                _ => (),
            }
        }
        map_2x.push(vec);
    }
    map_2x
}
pub(crate) fn day15(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("file should exist");
    let mut lines = input.lines().peekable();
    let mut map: Vec<Vec<u8>> = Vec::new();

    let mut y: i32 = -1;
    let mut x: i32 = -1;
    let mut _y = -1;
    while *lines.peek().expect("no empty line before directions") != "" {
        _y += 1;
        let line = *lines.peek().unwrap();
        if let Some(i) = line.find('@') {
            x = i as i32;
            y = _y;
        }
        map.push(line.bytes().collect());
        lines.next();
    }

    assert_ne!(x, -1);
    lines.next();
    let mut y_2x = y;
    let mut x_2x = 0;
    let mut map_2x = create_2x_map(&map, &mut x_2x);
    map_2x[y_2x as usize][x_2x as usize] = b'.';
    map[y as usize][x as usize] = b'.';

    while lines.peek().is_some() {
        let line = lines.peek().unwrap();
        for c in line.bytes() {
            (x, y) = move_robot(&mut map, translate_instruction(c), (x, y));
            (x_2x,y_2x) = move_robot_2x(&mut map_2x,translate_instruction(c),(x_2x,y_2x));
        }
        lines.next();
    }

    for v in &map {
        for c in v {
            print!("{}", *c as char);
        }
        println!();
    }

    for v in &map_2x{
        for c in v {
            print!("{}", *c as char);
        }
        println!();
    }
    println!("Solution for day 15 part 1: {}", count_score(&map, b'O'));
    println!("Solution for day 15 part 2: {}", count_score(&map_2x,b'['));
}
