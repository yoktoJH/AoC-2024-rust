use regex::{Captures, Regex};
use std::fs;

fn parse_mul(captures: &Captures, cap_grp1:usize, cap_grp2:usize) -> i32 {
    let a: i32 = captures[cap_grp1].parse().expect("something is very wrong");
    let b: i32 = captures[cap_grp2].parse().expect("something is very wrong pt2");
    a * b
}

pub(crate) fn day3(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("failed to read input file");
    let re = Regex::new(r"mul\(([0-9]|[1-9][0-9]{1,2}),([0-9]|[1-9][0-9]{1,2})\)").unwrap();
    let re2 =
        Regex::new(r"(mul\(([0-9]|[1-9][0-9]{1,2}),([0-9]|[1-9][0-9]{1,2})\)|do\(\)|don't\(\))")
            .unwrap();
    let mut result = 0;
    for mul in re.captures_iter(&input) {
        result += parse_mul(&mul,1,2);
    }
    let mut result2 = 0;
    let mut enabled = true;
    for inst in re2.captures_iter(&input) {
        if inst[0].eq("do()") {
            enabled = true;
        } else if inst[0].eq("don't()") {
            enabled = false;
        } else if enabled {
            result2 += parse_mul(&inst,2,3);
        }
    }
    println!("Solution for day 3 part 1: {result}");
    println!("Solution for day 3 part 2: {result2}");
}
