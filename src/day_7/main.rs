use crate::utils::read_lines;

fn parse_str_as_int_vec(s: String, pat: &str) -> Vec<i32> {
    s.split(pat)
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn sum_of_remaining_nums(nums: Vec<i32>) -> i32 {
    nums.iter().sum()
}

pub fn part_1(file: &str) -> i32 {
    let mut total: i32 = 0;
    for line in read_lines(file).map_while(Result::ok) {
        println!("{:?}", line);

        let mut parsed_line = line.split(": ");
        let target: i32 = parsed_line.next().unwrap().parse().expect("parse error");
        let mut nums: Vec<i32> = parse_str_as_int_vec(parsed_line.next().unwrap().to_string(), " ");

        let mut current_num: i32 = nums.pop().unwrap();
        while !nums.is_empty() {
            if current_num + sum_of_remaining_nums(nums.clone()) == target {
                println!("nailed it");
                total += target;
            } else if current_num + sum_of_remaining_nums(nums.clone()) < target {
                println!("need to do some multiplication");
                let next_num = nums.pop().unwrap();
                current_num *= next_num;
            } else {
                panic!("wtf");
            }
        }
    }
    total
}

pub fn part_2(_file: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_7/files/test.txt"), 0);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_7/files/input.txt"), 0);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_7/files/test.txt"), 0);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_7/files/input.txt"), 0);
    }
}
