const DIAL_NUMBERS_TOTAL: usize = 100;
const TARGET_POSITION: usize = 0;
const DIAL_STARTING_POINT: usize = 50;
const PUZZLE_NUMBER: usize = 1;
const DIRECTION_SPLIT_INDEX: usize = 1;
const DIRECTION_RIGHT: &str = "R";
const DIRECTION_LEFT: &str = "L";
const DEFAULT_ROTATION: isize = 0;
const INITIAL_PASSWORD_COUNT: usize = 0;

use crate::utils;

fn convert_rotation_to_number(path: &str) -> isize {
    let (direction, value) = path.split_at(DIRECTION_SPLIT_INDEX);
    let rotation_value = value.parse::<isize>().unwrap_or(DEFAULT_ROTATION);
    match direction {
        DIRECTION_RIGHT => rotation_value,
        DIRECTION_LEFT => -rotation_value,
        _ => DEFAULT_ROTATION,
    }
}

pub fn solve() {
    let puzzle_input = utils::get_puzzle_input_path(PUZZLE_NUMBER);
    let puzzle_input = String::from_utf8_lossy(&puzzle_input);

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

    println!("--- DAY {PUZZLE_NUMBER} ---");
    println!("Final dial position is: {final_position}");
    println!("The real password is: {password_count}");
    println!("--------------");
}
