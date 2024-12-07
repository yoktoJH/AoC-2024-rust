use std::fs;

fn check_line(values: &Vec<&str>) -> bool {
    let mut sign: i32 = 0;
    let mut num: i32 = values[0].parse().expect("cannot convert number");

    for i in 1..values.len() {
        let next_num: i32 = values[i].parse().expect("unable to convert");
        let a_diff = num.abs_diff(next_num);
        if sign == 0 {
            sign = num - next_num;
        }
        if a_diff > 3
            || a_diff < 1
            || (sign != 0 && sign.is_positive() != (num - next_num).is_positive())
        {
            return false;
        }
        num = next_num;
    }
    true
}

fn check_line2(values: &Vec<&str>, skip_index: usize) -> bool {
    let mut start = 0;
    if skip_index == 0 {
        start = 1;
    }
    let mut sign: i32 = 0;
    let mut num: i32 = values[start].parse().expect("cannot convert number");
    for i in (start + 1)..values.len() {
        if i == skip_index {
            continue;
        }
        let next_num: i32 = values[i].parse().expect("unable to convert");
        let a_diff = num.abs_diff(next_num);
        if sign == 0 {
            sign = num - next_num;
        }
        if a_diff > 3
            || a_diff < 1
            || (sign != 0 && sign.is_positive() != (num - next_num).is_positive())
        {
            if skip_index == usize::MAX {
                return check_line2(values, i) || check_line2(values, i - 1);
            }
            return false;
        }
        num = next_num;
    }
    true
}

pub(crate) fn day2(file_path: &str) {
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file_content.lines();
    let mut safe_lines1 = 0;
    let mut safe_lines2 = 0;

    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        if check_line(&values) {
            safe_lines1 += 1;
            safe_lines2 += 1;
        } else if check_line2(&values, 0) || check_line2(&values, usize::MAX) {
            safe_lines2 += 1;
        }
    }
    println!("Solution for day 2 part 1 {safe_lines1}");
    println!("Solution for day 2 part 2: {safe_lines2}");
}
