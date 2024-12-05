use crate::utils::read_lines;
use std::collections::HashMap;

fn parse_str_as_int_vec(s: String, pat: &str) -> Vec<i32> {
    s.split(pat)
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

pub fn parse_input(file: &str) -> (HashMap<i32, Vec<i32>>, Vec<String>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages = vec![];
    for line in read_lines(file).flatten() {
        // If the line contains '|', we're in the rules section
        if line.contains("|") {
            // Parse the line into a vector of integers
            let rule: Vec<i32> = parse_str_as_int_vec(line, "|");
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
    (rules, pages)
}

fn is_page_valid(page: Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, num) in page.clone().into_iter().enumerate() {
        if rules.contains_key(&num) {
            for rule in rules.get(&num).unwrap() {
                // Check if the rule is in the page before the current number
                let index = page.iter().position(|&r| r == *rule);
                if index.is_some() && index.unwrap() < i {
                    return false;
                }
            }
        }
    }
    true
}

pub fn part_1(file: &str) -> i32 {
    let (rules, pages) = parse_input(file);
    let mut total = 0;

    // Iterate through the pages and check if they are valid
    for raw_page in pages {
        // Parse the page into a vector of integers
        let page: Vec<i32> = parse_str_as_int_vec(raw_page, ",");
        // If the page is valid, add the middle page number to the total
        if is_page_valid(page.clone(), &rules) {
            total += page[page.len().div_ceil(2) - 1];
        }
    }
    total
}

fn page_sorter(page: Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    println!("{:?}", page);
    for (i, num) in page.clone().into_iter().enumerate() {
        if rules.contains_key(&num) {
            for rule in rules.get(&num).unwrap() {
                // Check if the rule is in the page before the current number
                let index = page.iter().position(|&r| r == *rule);
                if index.is_some() && index.unwrap() < i {
                    // idk what to do here
                    // if (i+1) < page.len() {
                    //     page.swap(i, i+1);
                    // }
                    // page_sorter(page.clone(), rules);
                }
            }
        }
    }
    page
}

pub fn part_2(file: &str) -> i32 {
    let (rules, pages) = parse_input(file);
    let total = 0;

    // Iterate through the pages and check if they are invalid
    for raw_page in pages {
        // Parse the page into a vector of integers
        let page: Vec<i32> = parse_str_as_int_vec(raw_page, ",");
        if !is_page_valid(page.clone(), &rules) {
            let sorted_page = page_sorter(page, &rules);
            println!("{:?}", sorted_page);
        }
    }

    total
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
