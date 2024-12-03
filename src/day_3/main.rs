use crate::utils::read_lines;
use regex::Regex;

fn find_multipliers(text: String) -> Vec<Vec<i32>> {
    let mut multipliers = vec![];
    let re_mul = Regex::new(r"mul[(]\d+,\d+[)]").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();
    // Find all instances of "mul(<num>,<num>)"
    re_mul.find_iter(&text).for_each(|m| {
        multipliers.push(
            // Push each parsed i32 pair to the multipliers vector
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
        // For each line, find all instances of "mul(<num>,<num>)"
        for nums_to_multiply in find_multipliers(line) {
            // Multiply the two numbers and add to the total
            total += nums_to_multiply[0] * nums_to_multiply[1];
        }
    }

    total
}

pub fn part_2(file: &str) -> i32 {
    let mut total = 0;
    let mut vec: Vec<&str> = vec![];

    // Read the file as a single string
    let big_string: String = read_lines(file).flatten().collect();
    // Split the string by "do()" and "don't()" and push the first element of each split to a vector
    big_string.split("do()").for_each(|s| {
        let splits: Vec<&str> = s.split("don't()").collect();
        vec.push(splits[0]);
    });

    // For each element in the vector, find all instances of "mul(<num>,<num>)"
    for elem in vec {
        for nums_to_multiply in find_multipliers(elem.to_string()) {
            // Multiply the two numbers and add to the total
            total += nums_to_multiply[0] * nums_to_multiply[1];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_3/files/test_1.txt"), 161);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_3/files/input.txt"), 184122457);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_3/files/test_2.txt"), 48);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_3/files/input.txt"), 107862689);
    }
}
