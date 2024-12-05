pub fn part_1(file: &str) -> i32 {
    println!("part 1 in progress: {}", file);
    0
}

pub fn part_2(file: &str) -> i32 {
    println!("part 2 in progress: {}", file);
    0
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_5/files/test.txt"), 0);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_5/files/input.txt"), 0);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_5/files/test.txt"), 0);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_5/files/input.txt"), 0);
    }
}
