use crate::utils::read_lines;
use std::collections::HashMap;

pub fn part_1(file: &str) -> i32 {
    // Step 1: Set up the rules hashmap and pages vector
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages = vec![];
    for line in read_lines(file).flatten() {
        // If the line contains '|', we're in the rules section
        if line.contains("|") {
            // Parse the line into a vector of integers
            let rule: Vec<i32> = line
                .split('|')
                .map(|s| s.parse().expect("parse error"))
                .collect();
            // Build the rules hashmap
            if rules.contains_key(&rule[0]) {
                // If the key already exists, append the value to the vector
                let mut temp = rules.get(&rule[0]).unwrap().clone();
                temp.push(rule[1]);
                rules.insert(rule[0], temp);
            } else {
                rules.insert(rule[0], vec![rule[1]]);
            }

        // If the line contains ',', we're in the pages section
        } else if line.contains(",") {
            pages.push(line);
        }
    }

    // Step 2: Iterate through the pages and check if they are valid
    let mut total = 0;
    for raw_page in pages {
        // Parse the page into a vector of integers
        let page: Vec<i32> = raw_page
            .split(',')
            .map(|s| s.parse().expect("parse error"))
            .collect();

        let mut is_page_valid = true;
        'page_loop: for (i, num) in page.clone().into_iter().enumerate() {
            if rules.contains_key(&num) {
                for rule in rules.get(&num).unwrap() {
                    // Check if the rule is in the page before the current number
                    let index = page.iter().position(|&r| r == *rule);
                    if index.is_some() && index.unwrap() < i {
                        is_page_valid = false;
                        break 'page_loop;
                    }
                }
            }
        }
        // If the page is valid, add the middle page number to the total
        if is_page_valid {
            total += page[page.len().div_ceil(2) - 1];
        }
    }
    total
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
        assert_eq!(part_1("src/day_5/files/test.txt"), 143);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_5/files/input.txt"), 4814);
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
