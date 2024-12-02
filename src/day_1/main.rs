use crate::utils::read_lines;

fn helper(file: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let first = line.split_whitespace().next();
            let last = line.split_whitespace().clone().last();
            if first.is_some() {
                v1.push(first.unwrap().parse::<i32>().unwrap());
            }
            if last.is_some() {
                v2.push(last.unwrap().parse::<i32>().unwrap());
            }
        }
    }
    (v1, v2)
}

pub fn part_1(file: &str) -> i32 {
    let mut diff_sum: i32 = 0;

    let (mut v1, mut v2) = helper(file);
    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        diff_sum += (v1[i] - v2[i]).abs();
    }

    diff_sum
}

pub fn part_2(file: &str) -> i32 {
    let mut diff_sum: i32 = 0;

    let (v1, v2) = helper(file);

    for i in 0..v1.len() {
        diff_sum += v1[i] * v2.iter().filter(|&n| *n == v1[i.clone()]).count() as i32;
    }

    diff_sum
}

#[cfg(test)]
mod tests {
    use crate::day_1::main::{part_1, part_2};

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_1/files/test.txt"), 11);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_1/files/input.txt"), 1889772);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_1/files/test.txt"), 31);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_1/files/input.txt"), 23228917);
    }
}
