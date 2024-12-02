use crate::utils::read_lines;

pub fn main(file: &str) -> i32 {
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
            diff_sum += (v1[i] * v2.iter().filter(|&n| *n == v1[i.clone()]).count() as i32);
        }
    }
    diff_sum
}

#[cfg(test)]
mod tests {
    use crate::day_1::part_2::main;

    #[test]
    fn test_sample_file() {
        assert_eq!(main("src/day_1/files/test_1.txt"), 31);
    }
    #[test]
    fn test_real_file() {
        assert_eq!(main("src/day_1/files/input.txt"), 23228917);
    }
}
