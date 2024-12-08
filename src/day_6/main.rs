use crate::utils::read_lines;
use std::collections::HashSet;

fn get_next_step(guard: ((i32, i32), char), obstacles: Vec<(i32, i32)>) -> ((i32, i32), char) {
    let mut next_guard = guard;

    match guard.1 {
        '^' => next_guard.0 .0 -= 1,
        'v' => next_guard.0 .0 += 1,
        '<' => next_guard.0 .1 -= 1,
        '>' => next_guard.0 .1 += 1,
        _ => (),
    }

    if obstacles.contains(&next_guard.0) {
        next_guard = match guard.1 {
            '^' => (guard.0, '>'),
            'v' => (guard.0, '<'),
            '<' => (guard.0, '^'),
            '>' => (guard.0, 'v'),
            _ => next_guard,
        };
    }

    next_guard
}

fn setup_map(map: Vec<String>) -> (((i32, i32), char), Vec<(i32, i32)>) {
    let mut guard: ((i32, i32), char) = ((0, 0), '^');
    let mut obstacles: Vec<(i32, i32)> = vec![];
    for (x, row) in map.iter().enumerate() {
        for (y, col) in row.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if col == '#' {
                obstacles.push((x, y));
            }
            if col == '^' || col == 'v' || col == '<' || col == '>' {
                guard = ((x, y), col);
            }
        }
    }
    (guard, obstacles)
}

pub fn part_1(file: &str) -> i32 {
    let mut seen_coords: HashSet<(i32, i32)> = HashSet::new();
    let map = read_lines(file).flatten().collect::<Vec<String>>();
    let (mut guard, obstacles) = setup_map(map.clone());

    while guard.0 .0 >= 0
        && guard.0 .0 < map[0].len() as i32
        && guard.0 .1 >= 0
        && guard.0 .1 < map[0].len() as i32
    {
        seen_coords.insert(guard.0);
        guard = get_next_step(guard, obstacles.clone());
    }

    seen_coords.len() as i32
}

pub fn part_2(_file: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_6::*;

    #[test]
    fn part_1_test_file() {
        assert_eq!(part_1("src/day_6/files/test.txt"), 41);
    }
    #[test]
    fn part_1_input_file() {
        assert_eq!(part_1("src/day_6/files/input.txt"), 4433);
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_6/files/test.txt"), 6);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_6/files/input.txt"), 0);
    }
}
