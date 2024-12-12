mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    println!("Hello, world!");
    day01::day1("inputs/input-1.txt");
    println!("--------------------------------------");
    day02::day2("inputs/input-2.txt");
    println!("--------------------------------------");
    day03::day3("inputs/input-3.txt");
    println!("--------------------------------------");
    day04::day4("inputs/input-4.txt");
    println!("--------------------------------------");
    day05::day5("inputs/input-5.txt");
    println!("--------------------------------------");
    day06::day6("inputs/input-6.txt");
    println!("--------------------------------------");
}
