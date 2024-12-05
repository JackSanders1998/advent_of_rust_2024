use crate::utils::read_lines;

pub fn part_1(file: &str) -> i32 {
    println!("part 1 in progress: {}", file);
    let mut rule_a = vec![];
    let mut rule_b = vec![];
    let mut pages = vec![];
    for line in read_lines(file).flatten() {
        if line.contains("|") {
            let rule: Vec<i32> = line.split('|').map(|s| s.parse().expect("parse error")).collect();
                rule_a.push(rule[0]);
                rule_b.push(rule[1]);
        } else if line.contains(","){
            pages.push(line);
        }
    }

    for page in pages {
        println!("=======new page========");
        let parsed_page: Vec<i32> = page.split(',').map(|s| s.parse().expect("parse error")).collect();
        for (i, num) in parsed_page.into_iter().enumerate() {
            let index_a = rule_a
                .iter()
                .enumerate()
                .filter_map(|(index, &r)| (r == num).then(|| index))
                .collect::<Vec<_>>();
            for index in index_a {
                let index_b = rule_a.iter().position(|&r| r == num);
            }
            println!("{:?}", index_a);

            // let index_a = rule_a.iter().position(|&r| r == num);
            // let index_b = rule_b.iter().position(|&r| r == num);
            // if index_a.is_some() && index_b.is_some(){
            //     if index_a > index_b {
            //         println!("{:?}, {:?}", index_a.unwrap(), index_b.unwrap());
            //         println!("error");
            //     }
            // }
        }
    }
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
