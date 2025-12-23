use crate::utils;

use super::parse_range;

const PUZZLE_NUMBER: usize = 5;
const SPLIT_DELIMITER: &str = "\n\n";

// Binary-search membership check against merged intervals
fn is_id_in_intervals(merged: &[(usize, usize)], id: usize) -> bool {
    let mut left = 0;
    let mut right = merged.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let (start, end) = merged[mid];

        if id < start {
            right = mid;
        } else if id > end {
            left = mid + 1;
        } else {
            return true; // ID is within the range
        }
    }

    false // ID not found in any range
}

fn solve_part1(puzzle_input: &str) -> usize {
    let mut parts = puzzle_input.splitn(2, SPLIT_DELIMITER);
    let fresh_ingredients_id_ranges = parts.next().unwrap_or("");
    let ids_to_check = parts.next().unwrap_or("");

    // Collect ranges
    let mut ranges: Vec<(usize, usize)> = fresh_ingredients_id_ranges
        .split_whitespace()
        .filter_map(parse_range)
        .map(|r| (*r.start(), *r.end()))
        .collect();

    if ranges.is_empty() {
        return 0;
    }

    // Sort and merge overlapping/adjacent ranges into disjoint intervals
    ranges.sort_unstable_by_key(|(start, _)| *start);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    let mut current_range = ranges[0];

    for range in ranges.iter() {
        if range.0 <= current_range.1 + 1 {
            // Overlapping or adjacent ranges, merge them
            current_range.1 = current_range.1.max(range.1);
        } else {
            // Non-overlapping range, push the current range and start a new one
            merged.push(current_range);
            current_range = *range;
        }
    }
    merged.push(current_range);

    let ids_to_check_iter = ids_to_check
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());

    let fresh_ids_count = ids_to_check_iter
        .filter(|id| is_id_in_intervals(&merged, *id))
        .count();
    println!("Part 1 - Number of fresh IDs: {fresh_ids_count}");

    fresh_ids_count
}

fn solve_part2(puzzle_input: &str) -> usize {
    let mut parts = puzzle_input.splitn(2, SPLIT_DELIMITER);
    let fresh_ingredients_id_ranges = parts.next().unwrap_or("");

    // Collect ranges (do not expand them to individual IDs)
    let mut ranges: Vec<(usize, usize)> = fresh_ingredients_id_ranges
        .split_whitespace()
        .filter_map(parse_range)
        .map(|r| (*r.start(), *r.end()))
        .collect();

    if ranges.is_empty() {
        return 0;
    }

    // Sort and merge overlapping/adjacent ranges into disjoint intervals
    ranges.sort_unstable_by_key(|(start, _)| *start);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    let mut current_range = ranges[0];

    for range in ranges.iter() {
        if range.0 <= current_range.1 + 1 {
            // Overlapping or adjacent ranges, merge them
            current_range.1 = current_range.1.max(range.1);
        } else {
            // Non-overlapping range, push the current range and start a new one
            merged.push(current_range);
            current_range = *range;
        }
    }
    merged.push(current_range);

    let mut number_of_fresh_ingredient_id = 0;

    for range in merged.iter() {
        number_of_fresh_ingredient_id += range.1 - range.0 + 1;
    }

    println!(
        "Part 2 - Number of ingredient IDs considered to be fresh: {number_of_fresh_ingredient_id}\n"
    );

    number_of_fresh_ingredient_id
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
        let fresh_ids_count = solve_part1(puzzle_dummy_input);
        assert_eq!(fresh_ids_count, 3);
    }

    #[test]
    fn test_part2() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let number_of_fresh_ingredient_id = solve_part2(puzzle_dummy_input);
        assert_eq!(number_of_fresh_ingredient_id, 14);
    }
}
