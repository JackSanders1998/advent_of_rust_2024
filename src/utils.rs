// use std::fs::read_to_string;
// use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// pub fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();
//
//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string())
//     }
//
//     result
// }
//
// pub fn get_all_digits(line: &str) -> Vec<i64> {
//     let ref re: Regex = Regex::new(r"\d").unwrap();
//
//     re.find_iter(line)
//         .filter_map(|digit| digit.as_str().parse().ok())
//         .collect()
// }

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
