use crate::utils::read_lines;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    INCREASING,
    DECREASING,
    INVALID,
}

fn get_direction(a: i32, b: i32) -> Direction {
    let diff = b - a;
    if diff.abs() > 0 && diff.abs() < 4 {
        if diff > 0 {
            Direction::INCREASING
        } else {
            Direction::DECREASING
        }
    } else {
        Direction::INVALID
    }
}

fn parse_line_as_int_vec(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn is_safe_row(numbers: Vec<i32>) -> bool {
    // Set the initial direction (increasing, decreasing, or invalid)
    let direction: Direction = get_direction(numbers[0], numbers[1]);
    for (i, num) in numbers.iter().enumerate().skip(1) {
        let new_direction = get_direction(numbers[i - 1], *num);
        if new_direction == Direction::INVALID || new_direction != direction {
            return false;
        }
    }
    // If the sequence is safe after looping through all nums, return true
    true
}

pub fn part_1(file: &str) -> i32 {
    let mut safe_count: i32 = 0;

    for line in read_lines(file).flatten() {
        if is_safe_row(parse_line_as_int_vec(line)) {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn part_2(file: &str) -> i32 {
    let mut safe_count: i32 = 0;

    for line in read_lines(file).flatten() {
        // Get vector of i32's from the line
        let numbers = parse_line_as_int_vec(line);

        if is_safe_row(numbers.clone()) {
            safe_count += 1;
            continue;
        }

        for i in 0..numbers.len() {
            let mut numbers_copy = numbers.clone();
            numbers_copy.remove(i);
            if is_safe_row(numbers_copy.clone()) {
                safe_count += 1;
                break;
            }
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_2/files/test.txt"), 2);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_2/files/input.txt"), 559);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_2/files/test.txt"), 4);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_2/files/input.txt"), 601);
    }
}
