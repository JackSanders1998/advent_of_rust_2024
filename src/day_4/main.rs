use crate::utils::read_lines;

fn coords_to_check(x: i32, y: i32) -> Vec<Vec<(i32, i32)>> {
    let mut coords = vec![];
    // diagonals
    coords.push(vec![(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]);
    coords.push(vec![(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]);
    coords.push(vec![(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]);
    coords.push(vec![(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]);
    // horizontals
    coords.push(vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]);
    coords.push(vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)]);
    // verticals
    coords.push(vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]);
    coords.push(vec![(x, y), (x - 1, y), (x - 2, y), (x - 3, y)]);
    coords
}

fn coord_lookup(coords: Vec<(i32, i32)>, word_vec: Vec<String>, part: &str) -> bool {
    let mut word = "".to_string();
    for coord in coords {
        // check if coord is within bounds
        if coord.0 >= 0
            && coord.0 < word_vec[0].len() as i32
            && coord.1 >= 0
            && coord.1 < word_vec.len() as i32
        {
            word += word_vec[coord.0 as usize]
                .chars()
                .nth(coord.1 as usize)
                .unwrap()
                .to_string()
                .as_str();
        }
    }
    // check if word is XMAS (or MAS for part 2) forwards or backwards
    if word == (if part == "part_1" { "XMAS" } else { "MAS" })
        || word == (if part == "part_1" { "SAMX" } else { "SAM" })
    {
        return true;
    }
    false
}

pub fn part_1(file: &str) -> i32 {
    let mut xmas_count = 0;
    let word_vec = read_lines(file).flatten().collect::<Vec<String>>();
    for (x, row) in word_vec.iter().enumerate() {
        for (y, letter) in row.chars().enumerate() {
            if letter == 'X' {
                let coords = coords_to_check(x as i32, y as i32);
                for coord in coords {
                    if coord_lookup(coord.clone(), word_vec.clone(), "part_1") {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    xmas_count
}

pub fn part_2(file: &str) -> i32 {
    let mut xmas_count = 0;
    let word_vec = read_lines(file).flatten().collect::<Vec<String>>();
    for (x, row) in word_vec.iter().enumerate() {
        for (y, letter) in row.chars().enumerate() {
            if letter == 'A' {
                let x = x as i32;
                let y = y as i32;
                let coords: Vec<Vec<(i32, i32)>> = vec![
                    vec![(x - 1, y - 1), (x, y), (x + 1, y + 1)],
                    vec![(x + 1, y - 1), (x, y), (x - 1, y + 1)],
                ];
                let mut count = 0;
                for coord in coords {
                    if coord_lookup(coord.clone(), word_vec.clone(), "part_2") {
                        count += 1;
                    }
                }
                if count == 2 {
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

#[cfg(test)]
mod tests {
    use crate::day_4::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_4/files/test.txt"), 18);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_4/files/input.txt"), 2464);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_4/files/test.txt"), 9);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_4/files/input.txt"), 1982);
    }
}
