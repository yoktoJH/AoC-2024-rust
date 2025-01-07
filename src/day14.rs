use std::fs;
use std::fs::File;
use std::io::Write;

fn numbers_from_slice(slice: &str) -> (i64, i64) {
    let mut first_split = slice.split("=");
    let mut second_split = first_split.nth(1).unwrap().split(",");
    let x: i64 = second_split
        .nth(0)
        .expect("There should be + to split the text")
        .parse()
        .expect("this should be a number");
    let y: i64 = second_split
        .nth(0)
        .expect("There should be + to split the text")
        .parse()
        .expect("this should be a number");
    (x, y)
}

fn print_robots(robots:&Vec<Robot>,width:usize,height:usize,file: & mut File){
    for i in 0..height {
        let mut line = Vec::new();
        for j in 0..width{

            let mut c = b'.';
            for robot in robots{
                if robot.px as usize == j && robot.py as usize == i {
                    // here print robot
                    c = b'X';
                    break;
                }
            }
            line.push(c);
        }
        line.push(b'\n');
        let _ = file.write_all(line.as_slice());
    }
    let _ =file.write_all(b"------------------------------------------------------------------\n");
}

fn simulate_moves(robots:& mut Vec<Robot>,file: & mut File){
    for i in 0..1000{
        for robot in &mut *robots {
            let mut x = (robot.px + robot.vx*1) % (MIDDLE_COL * 2 + 1);
            let mut y = (robot.py + robot.vy*1) % (MIDDLE_ROW * 2 + 1);
            if x < 0 {
                x += MIDDLE_COL * 2 + 1;
            }
            if y < 0 {
                y += MIDDLE_ROW * 2 + 1;
            }
            robot.px = x;
            robot.py = y;
        }
        print_robots( &robots,(MIDDLE_COL * 2 + 1)as usize,(MIDDLE_ROW * 2 + 1) as usize,file);
        println!("done {i}");
    }
}
static MIDDLE_ROW: i64 = 51;
static MIDDLE_COL: i64 = 50;
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}
pub(crate) fn day14(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("Path should point to a valid file");
    let mut quadrants = [0, 0, 0, 0];
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let (px, py) = numbers_from_slice(split.next().unwrap());
        let (vx, vy) = numbers_from_slice(split.next().unwrap());
        robots.push(Robot { px, py, vx, vy });
        let mut x = (px + vx * 100) % (MIDDLE_COL * 2 + 1);
        let mut y = (py + vy * 100) % (MIDDLE_ROW * 2 + 1);
        if x < 0 {
            x += MIDDLE_COL * 2 + 1;
        }
        if y < 0 {
            y += MIDDLE_ROW * 2 + 1;
        }
        if x == MIDDLE_COL || y == MIDDLE_ROW {
            continue;
        }
        if x < MIDDLE_COL {
            if y < MIDDLE_ROW {
                quadrants[0] += 1;
            } else {
                quadrants[2] += 1;
            }
        } else {
            if y < MIDDLE_ROW {
                quadrants[1] += 1;
            } else {
                quadrants[3] += 1;
            }
        }
    }
    let mut file = File::create("output.txt").unwrap();
    simulate_moves(& mut robots, & mut file);
    println!(
        "Solution for day 14 part 1: {}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );
    println!("Solution for day 14 part 2: {}", 0);
}
