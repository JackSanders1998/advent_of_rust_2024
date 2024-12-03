use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    match file_to_buf_reader(filename) {
        Ok(lines) => lines,
        Err(_) => panic!("Could not read file"),
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn file_to_buf_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
