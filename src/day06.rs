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
    obs_row: i32,
    obs_col: i32,
    forbidden: (i32, i32),
    lab: &mut  Vec<Vec<u8>>,
) -> bool {
    if (obs_row,obs_col) == forbidden {
        return false;
    }

    let mut local_history: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

    let mut direction = Direction::Up;

    let old_tile = lab[obs_row as usize][obs_col as usize];
    lab[obs_row as usize][obs_col as usize] = b'#';
    let (mut row, mut col) = forbidden;
    loop {
        if !local_history.contains_key(&(row, col)) {
            local_history.insert((row, col), HashSet::new());
        }

        let (mut next_row, mut next_col) = direction.move_guard(row, col);

        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            lab[obs_row as usize][obs_col as usize] = old_tile;
            return false;
        }


        if local_history[&(row, col)].contains(&direction)
        {
            break;
        }

        local_history
            .get_mut(&(row, col))
            .unwrap()
            .insert(direction);

        let mut tile_in_front = lab[next_row as usize][next_col as usize];

        while b'#' == tile_in_front {
            direction = direction.rotate();
            (next_row, next_col) = direction.move_guard(row, col);

            if next_row < 0
                || next_col < 0
                || next_row >= lab.len() as i32
                || next_col >= lab[next_row as usize].len() as i32
            {
                lab[obs_row as usize][obs_col as usize] = old_tile;
                return false;
            }
            tile_in_front = lab[next_row as usize][next_col as usize];
        }

        row = next_row;
        col = next_col;

    }
    lab[obs_row as usize][obs_col as usize] = old_tile;
    true
}

pub(crate) fn day6(file_path: &str) {

    let input = fs::read_to_string(file_path).expect("Unable to read input file 6");
    let lines = input.lines();

    let mut row = 0;
    let mut col = None;

    let mut lab: Vec<Vec<u8>> = Vec::new();
    let mut lab_history: HashSet<(i32,i32)> = HashSet::new();

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
    let mut direction = Direction::Up;
    let forbidden_place = (row, col);


    loop {

        lab_history.insert((row,col));

        if lab[row as usize][col as usize] != b'X' {
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
        }

        if next_row < 0
            || next_col < 0
            || next_row >= lab.len() as i32
            || next_col >= lab[next_row as usize].len() as i32
        {
            break;
        }

        row = next_row;
        col = next_col;
    }
    lab_history.remove(&forbidden_place);

    let mut valid_obstructions = 0;
    for (ob_row,ob_col) in &lab_history{
        if is_viable_obstruction(*ob_row,*ob_col,forbidden_place,&mut lab){
            valid_obstructions+=1;
        }
    }
    //plus 1 because I removed the starting position
    println!("Solution for day 6 part 1: {}",lab_history.len() +1);

    println!("Solution for day 6 part 2: {valid_obstructions}");

}
