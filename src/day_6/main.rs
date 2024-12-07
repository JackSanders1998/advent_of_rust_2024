use crate::utils::read_lines;

fn get_next_step(
    guard: ((i32, i32), char),
    obstacles: Vec<(i32, i32)>,
) -> (((i32, i32), char), bool) {
    let mut next_guard = guard;
    let mut should_increment = false;

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
    } else {
        should_increment = true;
    }

    (next_guard, should_increment)
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
    let mut total = 1;
    let map = read_lines(file).flatten().collect::<Vec<String>>();
    for row in map.iter() {
        println!("{}", row);
    }
    let (mut guard, obstacles) = setup_map(map.clone());

    while guard.0 .0 >= 0
        && guard.0 .0 < map[0].len() as i32
        && guard.0 .1 >= 0
        && guard.0 .1 < map[0].len() as i32
    {
        println!("{:?}", guard);
        let guard_status = get_next_step(guard, obstacles.clone());
        guard = guard_status.0;
        if guard_status.1 {
            total += 1;
        }
    }
    total
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
        assert_eq!(part_1("src/day_6/files/input.txt"), 0);
        // 4824 too high
    }
    #[test]
    fn part_2_test_file() {
        assert_eq!(part_2("src/day_6/files/test.txt"), 0);
    }
    #[test]
    fn part_2_input_file() {
        assert_eq!(part_2("src/day_6/files/input.txt"), 0);
    }
}
