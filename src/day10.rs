use std::collections::HashSet;
use std::fs;

fn insert_reachable(
    map: &Vec<Vec<usize>>,
    position: (i32, i32),
    next_level: &mut HashSet<(i32, i32)>,
    height:usize
) {
    let (x0, y0) = position;
    for (xdiff,ydiff) in [(1,0),(-1,0),(0,1),(0,-1)]{

            let x = x0 + xdiff;
            let y = y0 + ydiff;
            if y <0 || y as usize >= map.len() || x <0 || x as usize >= map[0].len() {
                continue;
            }
            if map[y as usize][x as usize]  == height+1 {
                next_level.insert((x,y));
            }

    }
}
fn insert_reachable2(
    map: &Vec<Vec<usize>>,
    position: (i32, i32),
    next_level: &mut Vec<(i32, i32)>,
    height:usize
) {
    let (x0, y0) = position;
    for (xdiff,ydiff) in [(1,0),(-1,0),(0,1),(0,-1)]{

            let x = x0 + xdiff;
            let y = y0 + ydiff;
            if y <0 || y as usize >= map.len() || x <0 || x as usize >= map[0].len() {
                continue;
            }
            if map[y as usize][x as usize]  == height+1 {
                next_level.push((x,y));
            }

    }
}

fn reachable_hilltops(map: &Vec<Vec<usize>>, starting_position: (i32, i32)) -> (usize,usize) {
    let mut height = 0;
    let mut current_level: HashSet<(i32, i32)> = HashSet::new();
    let mut current_level2 = Vec::new();
    current_level.insert(starting_position);
    current_level2.push(starting_position);
    while height < 9 {
        let mut next_level = HashSet::new();
        let mut next_level2 = Vec::new();
        for position in current_level {
            insert_reachable(&map, position, &mut next_level,height);
        }
        for position in current_level2{
            insert_reachable2(&map,position,&mut next_level2,height)
        }
        current_level = next_level;
        current_level2 = next_level2;
        height += 1;
    }
    (current_level.len() ,current_level2.len())
}

// this could be done very well with generics and traits
pub(crate) fn day10(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read from input file");
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        map.push(line.bytes().map(|c| (c - b'0') as usize).collect());
    }
    let mut score = 0;
    let  mut rank =0;
    for y in 0..map.len() {
        for x in 0..map[0].len(){
            if map[y][x] ==0 {
                let (s,r) =reachable_hilltops(&map,(x as i32,y as i32));
                //println!("rank for {:?} {}",(x,y),r);
                score += s;
                rank += r;
            }

        }
    }
    println!("Solution for day 10 part 1: {}", score);
    println!("Solution for day 10 part 2: {}", rank);
}
