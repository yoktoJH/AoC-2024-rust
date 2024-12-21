use std::collections::{HashMap, HashSet};
use std::fs;



fn find_antinodes(
    antinodes: &mut HashSet<(i32, i32)>,
    antinodes2: &mut HashSet<(i32, i32)>,
    antennas: &Vec<(i32, i32)>,
    height: i32,
    width: i32,
) {
    for i in 0..antennas.len()-1 {
        let ant1 =&antennas[i];
        for j in i+1..antennas.len(){
            let ant2 = &antennas[j];
            //just add the antennas themselves
            antinodes2.insert(ant1.clone());
            antinodes2.insert(ant2.clone());


            let width_diff = ant1.0 - ant2.0;
            let height_diff = ant1.1 - ant2.1;

            let mut anti_x = ant1.0+width_diff;
            let mut anti_y = ant1.1 + height_diff;
            if 0<= anti_x && anti_x < width && 0 <= anti_y && anti_y < height {
                antinodes.insert((anti_x,anti_y));

            }
            //part 2 loop
            while  0<= anti_x && anti_x < width && 0 <= anti_y && anti_y < height {
                antinodes2.insert((anti_x,anti_y));
                anti_x += width_diff;
                anti_y += height_diff;
            }

            anti_x = ant2.0 - width_diff;
            anti_y = ant2.1 - height_diff;
            if 0<= anti_x && anti_x < width && 0 <= anti_y && anti_y < height {
                antinodes.insert((anti_x,anti_y));
            }

            //part 2 loop
            while  0 <= anti_x && anti_x < width && 0 <= anti_y && anti_y < height {
                antinodes2.insert((anti_x,anti_y));
                anti_x -= width_diff;
                anti_y -= height_diff;
            }

        }
    }
}

pub(crate) fn day8(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read from input file");
    let mut lines = input.lines().peekable();
    let mut height = 0;
    let width = lines.peek().unwrap().len() as i32;
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes2: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in lines.enumerate() {
        height += 1;
        for (x, c) in line.bytes().enumerate() {
            if c == b'.' {
                continue;
            }
            if !antennas.contains_key(&c) {
                antennas.insert(c, Vec::new());
            }
            antennas.get_mut(&c).unwrap().push((x as i32, y as i32));
        }
    }
    for (_, same_antennas) in antennas {
        find_antinodes(&mut antinodes,&mut antinodes2,&same_antennas,height,width);
    }

    println!("Solution for day 8 part 1: {}",antinodes.len());
    println!("Solution for day 8 part 2: {}",antinodes2.len());
}
