use std::fs;

fn solvable_equation(result: i64, values: &Vec<i64>, index: usize) -> bool {
    if index == 0 {
        return result == values[0];
    }
    if result % values[index] == 0 && solvable_equation(result / values[index], &values, index - 1)
    {
        return true;
    }
    solvable_equation(result - values[index], &values, index - 1)
}

fn deconcat(num: i64, tail: i64) -> Option<i64> {
    let mut x = 1;
    while x <= tail {
        x *= 10;
    }
    if tail != num % x {
        return None;
    }
    Some(num / x)
}
fn solvable_equation_2(result: i64, values: &Vec<i64>, index: usize) -> bool {
    if index == 0 {
        return result == values[0];
    }

    if result % values[index] == 0
        && solvable_equation_2(result / values[index], &values, index - 1)
    {
        return true;
    }

    let new_result = deconcat(result, values[index]);
    if new_result.is_some() && solvable_equation_2(new_result.unwrap(), values, index - 1) {
        return true;
    }
    solvable_equation_2(result - values[index], &values, index - 1)
}

pub(crate) fn day7(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("Unable to read input-7.txt");
    let lines = input.lines();
    let mut calibration_result: i64 = 0;
    let mut calibration_result2: i64 = 0;

    for line in lines {
        let mut numbers = line.split(":");
        let result: i64 = numbers.next().unwrap().parse().unwrap();
        let values: Vec<i64> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| -> i64 { s.parse().unwrap() })
            .collect();

        if solvable_equation(result, &values, values.len() - 1) {
            calibration_result += result;
        }
        if solvable_equation_2(result, &values, values.len() - 1) {
            calibration_result2 += result;
        }

    }

    println!("Solution for day 7 part 1: {calibration_result}");
    println!("Solution for day 7 part 2: {calibration_result2}");
}
