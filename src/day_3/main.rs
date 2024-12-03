use crate::utils::read_lines;
use regex::Regex;

fn find_multipliers(text: String) -> Vec<Vec<i32>> {
    let mut multipliers = vec![];
    let re_mul = Regex::new(r"mul[(]\d+,\d+[)]").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();
    re_mul.find_iter(&text).for_each(|m| {
        multipliers.push(
            re_nums
                .find_iter(m.as_str())
                .filter_map(|digit| digit.as_str().parse::<i32>().ok())
                .collect(),
        );
    });
    multipliers
}

pub fn part_1(file: &str) -> i32 {
    let mut total = 0;
    for line in read_lines(file).flatten() {
        for nums_to_multiply in find_multipliers(line) {
            total += nums_to_multiply[0] * nums_to_multiply[1];
        }
    }
    total
}

pub fn part_2(file: &str) -> i32 {
    println!("Part 2 not implemented yet, {}", file);
    0
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_3/files/test.txt"), 161);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_3/files/input.txt"), 184122457);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_3/files/test.txt"), 0);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_3/files/input.txt"), 0);
    }
}
