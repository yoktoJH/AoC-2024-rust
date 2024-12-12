use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn is_correct_line(numbers: &Vec<i32>, forbidden_combination: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, num) in numbers.iter().enumerate() {
        for successor in &numbers[i + 1..] {
            if forbidden_combination[num].contains(successor) {
                return false;
            }
        }
    }
    true
}

fn correct_line_middle(
    numbers: &Vec<i32>,
    forbidden_combination: &HashMap<i32, HashSet<i32>>,
) -> i32 {
    let mut corrected_line: Vec<i32> = Vec::new();
    corrected_line.push(numbers[0]);
    for num in &numbers[1..] {
        let mut i = corrected_line.len();
        while i > 0 && !forbidden_combination[num].contains(&corrected_line[i - 1]) {
            i -= 1;
        }
        corrected_line.insert(i, *num);
    }
    corrected_line[corrected_line.len() / 2]
}
pub(crate) fn day5(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("Unable to read input file");
    let lines = input.lines();
    let mut forbidden_combination: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut middle_page_sum1 = 0;
    let mut middle_page_sum2 = 0;
    let mut second_stage = false;
    for line in lines {
        if second_stage {
            let str_numbers: Vec<&str> = line.split(",").collect();
            let numbers: Vec<i32> = str_numbers.iter().map(|s| s.parse().unwrap()).collect();
            if is_correct_line(&numbers, &forbidden_combination) {
                middle_page_sum1 += numbers[numbers.len() / 2];
            } else {
                middle_page_sum2 += correct_line_middle(&numbers, &forbidden_combination);
            }
        } else if line.len() > 2 && line.as_bytes()[2] == b'|' {
            let num1: i32 = line[0..2].parse().expect("invalid input");
            let num2: i32 = line[3..].parse().expect("invalid input");

            if !forbidden_combination.contains_key(&num2) {
                forbidden_combination.insert(num2, HashSet::new());
            }

            forbidden_combination
                .get_mut(&num2)
                .expect("hashmap somehow does not contain the key")
                .insert(num1);
        } else {
            second_stage = true;
        }
    }
    println!("Solution for day 5 part 1: {middle_page_sum1}");
    println!("Solution for day 5 part 2: {middle_page_sum2}");
}
