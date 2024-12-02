mod day_1;
mod day_2;
mod utils;

fn main() {
    // day 1
    println!("day1/part1: {}", day_1::part_1("src/day_1/files/input.txt"));
    println!("day1/part2: {}", day_1::part_2("src/day_1/files/input.txt"));
    // day 2
    println!("day2/part1: {}", day_2::part_1("src/day_2/files/input.txt"));
    println!("day2/part2: {}", day_2::part_2("src/day_2/files/input.txt"));
}
