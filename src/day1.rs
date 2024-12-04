
use std::fs;


pub(crate) fn day1(file_path:&str) {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines = content.lines();
    let mut vec_left:Vec<i32> = Vec::new();
    let mut vec_right:Vec<i32> = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace();
        vec_left.push( numbers.next().expect("not enough values").parse().expect("cant convert"));
        vec_right.push( numbers.next().expect("not enough values").parse().expect("cant convert"));
    }
    vec_left.sort();
    vec_right.sort();
    let mut i=0;
    let mut j=0;
    let mut solution_1 =0;
    let mut solution_2=0;
    while i < vec_left.len() {
        solution_1+= vec_left[i].abs_diff(vec_right[i]);
        while vec_right[j] < vec_left[i] {
            j+=1;
        }
        while vec_right[j] == vec_left[i] {
            solution_2+=vec_left[i];
            j+=1;
        }
        i+=1;
    }
    println!("Solution for day 1 part 1: {solution_1}");
    println!("Solution for day 1 part 2: {solution_2}");
}