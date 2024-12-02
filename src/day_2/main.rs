use crate::utils::read_lines;

#[derive(Eq, PartialEq, Debug)]
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

pub fn part_1(file: &str) -> i32 {
    let mut safe_count: i32 = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();

            let direction: Direction = get_direction(numbers[0], numbers[1]);
            let mut safe = true;

            if direction != Direction::INVALID {
                for (i, num) in numbers.iter().enumerate().skip(1) {
                    let new_direction = get_direction(numbers[i - 1], *num);
                    if new_direction == Direction::INVALID || new_direction != direction {
                        safe = false;
                        break;
                    }
                }
                if safe {
                    safe_count += 1;
                }
            }
        }
    }
    safe_count
}

pub fn part_2(file: &str) -> i32 {
    let mut safe_count: i32 = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();

            let direction: Direction = get_direction(numbers[0], numbers[1]);
            let mut safe = true;

            if direction != Direction::INVALID {
                println!("{:?}", numbers);
                for (i, num) in numbers.iter().enumerate().skip(1) {
                    let new_direction = get_direction(numbers[i - 1], *num);

                    if new_direction == Direction::INVALID || new_direction != direction {
                        println!(
                            "{:?} try with: {}; {:?} {:?} {:?} {:?}",
                            numbers,
                            numbers[i - 2],
                            numbers[i - 1],
                            *num,
                            new_direction,
                            direction
                        );
                        let new_direction = get_direction(numbers[i - 2], *num);
                        if new_direction == Direction::INVALID || new_direction != direction {
                            safe = false;
                            break;
                        }
                    }
                }
                if safe {
                    safe_count += 1;
                }
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
        assert_eq!(part_2("src/day_2/files/input.txt"), 569);
        //  613 590 574 569
    }
}
