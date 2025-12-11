use crate::utils;

const DIAL_NUMBERS_TOTAL: usize = 100;
const TARGET_POSITION: usize = 0;
const DIAL_STARTING_POINT: usize = 50;
const PUZZLE_NUMBER: usize = 1;
const DIRECTION_SPLIT_INDEX: usize = 1;
const DIRECTION_RIGHT: &str = "R";
const DIRECTION_LEFT: &str = "L";
const DEFAULT_ROTATION: isize = 0;
const INITIAL_PASSWORD_COUNT: usize = 0;

fn convert_rotation_to_number(path: &str) -> isize {
    let (direction, value) = path.split_at(DIRECTION_SPLIT_INDEX);
    let rotation_value = value.parse::<isize>().unwrap_or(DEFAULT_ROTATION);
    match direction {
        DIRECTION_RIGHT => rotation_value,
        DIRECTION_LEFT => -rotation_value,
        _ => DEFAULT_ROTATION,
    }
}

pub fn solve_part1(puzzle_input: &str) -> usize {
    let (final_position, password_count) = puzzle_input
        .split_whitespace()
        .map(convert_rotation_to_number)
        .fold(
            (DIAL_STARTING_POINT, INITIAL_PASSWORD_COUNT),
            |(acc, count), x| {
                let new_pos = (acc as isize + x).rem_euclid(DIAL_NUMBERS_TOTAL as isize) as usize;
                (
                    new_pos,
                    if new_pos == TARGET_POSITION {
                        count + 1
                    } else {
                        count
                    },
                )
            },
        );

    println!("Part 1 - Final dial position is: {final_position}");
    println!("Part 1 - The real password is: {password_count}");
    password_count
}

pub fn solve_part2(puzzle_input: &str) -> usize {
    let (final_position, password_count) = puzzle_input
        .split_whitespace()
        .map(convert_rotation_to_number)
        .fold(
            (DIAL_STARTING_POINT, INITIAL_PASSWORD_COUNT),
            |(acc, count), x| {
                let temp_pos = acc as isize + x;
                let new_pos = temp_pos.rem_euclid(DIAL_NUMBERS_TOTAL as isize) as usize;
                (
                    new_pos,
                    count
                        + (temp_pos.unsigned_abs() / DIAL_NUMBERS_TOTAL)
                        + if temp_pos <= TARGET_POSITION as isize && acc != TARGET_POSITION {
                            1
                        } else {
                            0
                        },
                )
            },
        );

    println!("Part 2 - Final dial position is: {final_position}");
    println!("Part 2 - The real password is: {password_count}\n");
    password_count
}

pub fn solve() {
    println!("--- DAY {PUZZLE_NUMBER} ---");
    let puzzle_input = utils::get_puzzle_input_path(PUZZLE_NUMBER);
    let puzzle_input = str::from_utf8(&puzzle_input).unwrap_or("");
    solve_part1(puzzle_input);
    solve_part2(puzzle_input);
}

#[cfg(test)]
mod tests {
    use crate::utils::get_puzzle_dummy_input_path;

    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let password_count = solve_part1(puzzle_dummy_input);
        assert_eq!(password_count, 3);
    }

    #[test]
    fn test_part2() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let password_count = solve_part2(puzzle_dummy_input);
        assert_eq!(password_count, 6);
    }
}
