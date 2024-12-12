use std::collections::{HashMap, HashSet};
use std::fs;
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    //I assume not modifying input parameters is the Rust way
    fn move_guard(&self, row: i32, col: i32) -> (i32, i32) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }
}

fn is_viable_obstruction(
    mut direction: Direction,
    lab_history: &HashMap<(i32, i32), HashSet<Direction>>,
    mut row: i32,
    mut col: i32,
    forbidden: &(i32, i32),
    lab: &mut Vec<Vec<u8>>,
) -> bool {
    let mut local_history: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

    let obstruction_pos = direction.move_guard(row, col);
    if obstruction_pos == *forbidden {
        return false;
    }
    let old = lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize];
    lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = b'#';
    loop {
        if !local_history.contains_key(&(row, col)) {
            local_history.insert((row, col), HashSet::new());
        }

        if local_history[&(row, col)].contains(&direction)
            || (lab_history.contains_key(&(row, col))
                && lab_history[&(row, col)].contains(&direction))
        {
            lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = old;
            break;
        }

        local_history
            .get_mut(&(row, col))
            .unwrap()
            .insert(direction);

        let (mut next_row, mut next_col) = direction.move_guard(row, col);

        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = old;
            return false;
        }

        let tile_in_front = lab[next_row as usize][next_col as usize];

        if b'#' == tile_in_front {
            direction = direction.rotate();
            (next_row, next_col) = direction.move_guard(row, col)
        }

        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = old;
            return false;
        }

        row = next_row;
        col = next_col;
    }

    lab[obstruction_pos.0 as usize][obstruction_pos.1 as usize] = old;
    true
}

pub(crate) fn day6(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("Unable to read input file 6");
    let lines = input.lines();
    let mut row = 0;
    let mut col = None;
    let mut lab: Vec<Vec<u8>> = Vec::new();
    let mut lab_history: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

    for (i, line) in lines.enumerate() {
        lab.push(line.as_bytes().to_vec());
        let tmp_col = line.find("^");
        if tmp_col != None {
            col = tmp_col;
            row = i;
        }
    }

    let mut row = row as i32;
    let mut col = col.expect("Nothing was found, that is really bad!") as i32;
    let mut uniquely_visited_tiles = 0;
    let mut direction = Direction::Up;
    let forbidden_place = (row, col);
    let mut obstructions:HashSet<(i32,i32)> = HashSet::new();
    loop {
        if !lab_history.contains_key(&(row, col)) {
            lab_history.insert((row, col), HashSet::new());
        }

        if lab[row as usize][col as usize] != b'X' {
            uniquely_visited_tiles += 1;
            lab[row as usize][col as usize] = b'X';
        }
        let (mut next_row, mut next_col) = direction.move_guard(row, col);
        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            break;
        }

        let tile_in_front = lab[next_row as usize][next_col as usize];
        if b'#' == tile_in_front {
            direction = direction.rotate();
            (next_row, next_col) = direction.move_guard(row, col)
        } else if is_viable_obstruction(
            direction,
            &lab_history,
            row,
            col,
            &forbidden_place,
            &mut lab,
        ) {
           obstructions.insert((row,col));
        }

        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            break;
        }

        lab_history.get_mut(&(row, col)).unwrap().insert(direction);
        row = next_row;
        col = next_col;
    }

    println!("Solution for day 6 part 1: {uniquely_visited_tiles}");
    //1600 too high and 239 is too low
    println!("Solution for day 6 part 2: {}",obstructions.len());
}
