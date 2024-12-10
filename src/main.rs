extern crate core;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;
mod day_10;

fn main() {
    // day 1
    println!("day1/part1: {}", day_1::part_1("src/day_1/files/input.txt"));
    println!("day1/part2: {}", day_1::part_2("src/day_1/files/input.txt"));
    // day 2
    println!("day2/part1: {}", day_2::part_1("src/day_2/files/input.txt"));
    println!("day2/part2: {}", day_2::part_2("src/day_2/files/input.txt"));
    // day 3
    println!("day3/part1: {}", day_3::part_1("src/day_3/files/input.txt"));
    println!("day3/part2: {}", day_3::part_2("src/day_3/files/input.txt"));
    // day 4
    println!("day4/part1: {}", day_4::part_1("src/day_4/files/input.txt"));
    println!("day4/part2: {}", day_4::part_2("src/day_4/files/input.txt"));
    // day 5
    println!("day5/part1: {}", day_5::part_1("src/day_5/files/input.txt"));
    println!("day5/part2: {}", day_5::part_2("src/day_5/files/input.txt"));
    // day 6
    println!("day6/part1: {}", day_6::part_1("src/day_6/files/input.txt"));
    println!("day6/part2: {}", day_6::part_2("src/day_6/files/input.txt"));
    // day 7
    println!("day7/part1: {}", day_7::part_1("src/day_7/files/input.txt"));
    println!("day7/part2: {}", day_7::part_2("src/day_7/files/input.txt"));
    // day 8
    println!("day8/part1: {}", day_8::part_1("src/day_8/files/input.txt"));
    println!("day8/part2: {}", day_8::part_2("src/day_8/files/input.txt"));
    // day 9
    println!("day9/part1: {}", day_9::part_1("src/day_9/files/input.txt"));
    println!("day9/part2: {}", day_9::part_2("src/day_9/files/input.txt"));
    // day 10
    println!("day10/part1: {}", day_10::part_1("src/day_10/files/input.txt"));
    println!("day10/part2: {}", day_10::part_2("src/day_10/files/input.txt"));
}
