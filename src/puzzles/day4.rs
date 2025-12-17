use crate::{grid::Grid, utils};

const PUZZLE_NUMBER: usize = 4;
const NEIGHBOR_ROLL_THRESHOLD: usize = 4;
const ROLL_BYTE: u8 = b'@';
const EMPTY_BYTE: u8 = b'.';

fn solve_part1(puzzle_input: &str) -> usize {
    let grid = Grid::<u8>::parse(puzzle_input).unwrap();

    let mut n_accessed_rolls = 0;
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            if grid.get(row, column) != Some(&ROLL_BYTE) {
                continue;
            }
            let neighbors_rolls_count = grid
                .find_neighbors(row, column)
                .iter()
                .filter(|&coord| *coord == ROLL_BYTE)
                .count();
            if neighbors_rolls_count < NEIGHBOR_ROLL_THRESHOLD {
                n_accessed_rolls += 1;
            }
        }
    }

    println!("Part 1 - Number of accessed rolls by forklift: {n_accessed_rolls}");
    n_accessed_rolls
}

fn solve_part2(puzzle_input: &str) -> usize {
    let mut grid = Grid::<u8>::parse(puzzle_input).unwrap();

    let mut total_removed_rolls = 0;
    loop {
        let mut n_accessed_rolls = 0;
        for row in 0..grid.rows() {
            for column in 0..grid.columns() {
                if grid.get(row, column) != Some(&ROLL_BYTE) {
                    continue;
                }
                let neighbors_rolls_count = grid
                    .find_neighbors(row, column)
                    .iter()
                    .filter(|&coord| *coord == ROLL_BYTE)
                    .count();
                if neighbors_rolls_count < NEIGHBOR_ROLL_THRESHOLD {
                    n_accessed_rolls += 1;
                    if let Some(current_roll) = grid.get_mut(row, column) {
                        *current_roll = EMPTY_BYTE;
                    }
                }
            }
        }
        if n_accessed_rolls == 0 {
            break;
        } else {
            total_removed_rolls += n_accessed_rolls;
        }
    }

    println!("Part 2 - Number of removed rolls: {total_removed_rolls}\n");
    total_removed_rolls
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
        let n_accessed_rolls = solve_part1(puzzle_dummy_input);
        assert_eq!(n_accessed_rolls, 13);
    }

    #[test]
    fn test_part2() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let total_removed_rolls = solve_part2(puzzle_dummy_input);
        assert_eq!(total_removed_rolls, 43);
    }
}
