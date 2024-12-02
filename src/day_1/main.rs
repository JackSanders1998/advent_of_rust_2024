use crate::utils::read_lines;

pub fn part_1(file: &str) -> i32 {
    let mut diff_sum: i32 = 0;

    if let Ok(lines) = read_lines(file) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

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

        v1.sort();
        v2.sort();

        for i in 0..v1.len() {
            diff_sum += (v1[i] - v2[i]).abs();
        }
    }
    diff_sum
}

pub fn part_2(file: &str) -> i32 {
    let mut diff_sum: i32 = 0;

    if let Ok(lines) = read_lines(file) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

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

        for i in 0..v1.len() {
            diff_sum += v1[i] * v2.iter().filter(|&n| *n == v1[i.clone()]).count() as i32;
        }
    }
    diff_sum
}

#[cfg(test)]
mod tests {
    use crate::day_1::main::{part_1, part_2};

    #[test]
    fn test_sample_file() {
        assert_eq!(part_1("src/day_1/files/test.txt"), 11);
    }
    #[test]
    fn test_real_file() {
        assert_eq!(part_1("src/day_1/files/input.txt"), 1889772);
    }
    #[test]
    fn test_sample_file_2() {
        assert_eq!(part_2("src/day_1/files/test.txt"), 31);
    }
    #[test]
    fn test_real_file_2() {
        assert_eq!(part_2("src/day_1/files/input.txt"), 23228917);
    }
}
