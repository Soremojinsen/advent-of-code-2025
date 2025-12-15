use std::cmp::Ordering;

use crate::utils;

const PUZZLE_NUMBER: usize = 3;
const PART2_BATTERY_NUM: usize = 12;
const DIGIT_RADIX: u32 = 10;

fn largest_joltage_for_bank_with_two_batteries(bank: &str) -> usize {
    let bank_length = bank.len();
    let digits = bank
        .chars()
        .filter_map(|x| x.to_digit(DIGIT_RADIX))
        .enumerate()
        .fold((0, 0), |(first_digit, second_digit), (i, num)| {
            match (
                (i + 1).cmp(&bank_length),
                num.cmp(&first_digit),
                num.cmp(&second_digit),
            ) {
                (Ordering::Less, Ordering::Greater, _) => (num, 0),
                (Ordering::Less, Ordering::Equal | Ordering::Less, Ordering::Greater) => {
                    (first_digit, num)
                }
                (Ordering::Equal, _, Ordering::Greater) => (first_digit, num),
                _ => (first_digit, second_digit),
            }
        });
    (digits.0 * 10 + digits.1) as usize
}

fn largest_joltage_for_bank_with_n_batteries(bank: &str, batt: usize) -> usize {
    // We iterate the input digits (with indices) and maintain a buffer 
    // `digits` of length `batt` where each slot is the currently chosen
    // digit for that output position.
    let bank_length = bank.len();
    let (digits, _) = bank
        .chars()
        .filter_map(|x| x.to_digit(DIGIT_RADIX))
        .enumerate()
        .fold((vec![0u32; batt], 0usize), |(mut digits, mut digit_pos_state), (i, num)| {
            // Try to place `num` into the earliest position `j` it can occupy.
            // The condition `i < bank_length - batt + 1 + j` ensures we don't
            // pick a digit too late that would prevent filling the remaining
            // positions.
            for (j, value) in digits.iter_mut().enumerate() {
                if digit_pos_state >= j {
                    if num > *value && i < bank_length - batt + 1 + j {
                        *value = num;
                        digit_pos_state = j;
                        break;
                    }
                } else {
                    *value = num;
                    digit_pos_state = j;
                    break;
                }
            }
            (digits, digit_pos_state)
        });

    // Build numeric result from the chosen digits.
    let (value, _) = digits.into_iter().rev().fold((0usize, 1usize), |(acc, mul), v| {
        (acc + v as usize * mul, mul * (DIGIT_RADIX as usize))
    });
    value
}

fn solve_part1(puzzle_input: &str) -> usize {
    let total_output_joltage = puzzle_input
        .split_whitespace()
        .map(largest_joltage_for_bank_with_two_batteries)
        .sum();

    println!("Part 1 - Total output joltage: {total_output_joltage}");
    total_output_joltage
}

fn solve_part2(puzzle_input: &str) -> usize {
    let total_output_joltage = puzzle_input
        .split_whitespace()
        .map(|bank| largest_joltage_for_bank_with_n_batteries(bank, PART2_BATTERY_NUM))
        .sum();

    println!("Part 2 - Total output joltage: {total_output_joltage}\n");
    total_output_joltage
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
        let total_output_joltage = solve_part1(puzzle_dummy_input);
        assert_eq!(total_output_joltage, 357);
    }

    #[test]
    fn test_part2() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let total_output_joltage = solve_part2(puzzle_dummy_input);
        assert_eq!(total_output_joltage, 3121910778619);
    }
}
