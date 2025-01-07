use std::collections::HashMap;
use std::fs;

fn number_length(mut number: u64) -> usize {
    let mut length = 0;
    while number > 0 {
        length += 1;
        number /= 10;
    }
    return length;
}
fn stones_after_n_blinks(
    stone: u64,
    blinks_left: u64,
    memory: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if memory.contains_key(&(stone, blinks_left)) {
        return *memory.get(&(stone, blinks_left)).unwrap();
    }
    //apply rules
    if stone == 0 {
        let result = stones_after_n_blinks(1, blinks_left - 1, memory);
        memory.insert((stone, blinks_left), result);
        return result;
    }
    let length = number_length(stone);
    if length & 1 == 0 {
        let mut magic = 1;
        for _ in 0..(length / 2) {
            magic *= 10;
        }
        let result1 = stones_after_n_blinks(stone / magic, blinks_left - 1, memory);
        let result2 = stones_after_n_blinks(stone % magic, blinks_left - 1, memory);
        memory.insert((stone, blinks_left), result2 + result1);
        return result2 + result1;
    }
    let result = stones_after_n_blinks(stone * 2024, blinks_left - 1, memory);
    memory.insert((stone, blinks_left), result);
    result
}

pub(crate) fn day11(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("unable to read input file");
    let mut stone_count = 0;
    let mut stone_count2 =0;
    let mut memory: HashMap<(u64, u64), u64> = HashMap::new();
    for str_number in input.split_whitespace() {
        let number = str_number.parse().expect("Input is not a number");
        stone_count += stones_after_n_blinks(number, 25, &mut memory);
        stone_count2 += stones_after_n_blinks(number,75,&mut memory);
    }
    println!("Solution for day 11 part 1: {}", stone_count);
    println!("Solution for day 11 part 2: {}", stone_count2);
}
