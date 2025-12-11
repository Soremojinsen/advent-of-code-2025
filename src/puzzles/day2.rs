use crate::utils;

const PUZZLE_NUMBER: usize = 2;
const COMMA_DELIMITER: &str = ",";
const DASH_DELIMITER: &str = "-";

fn is_invalid_id_part1(id: usize) -> bool {
    let id_str = id.to_string();
    let mid = id_str.len() / 2;
    id_str[..mid] == id_str[mid..]
}

// Parse a single range fragment like "start-end" into a RangeInclusive<usize>
fn parse_range(fragment: &str) -> Option<std::ops::RangeInclusive<usize>> {
    let parts: Vec<&str> = fragment.split(DASH_DELIMITER).collect();
    if parts.len() != 2 {
        return None;
    }
    match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
        (Ok(s), Ok(e)) => Some(s..=e),
        _ => None,
    }
}

fn is_invalid_id_part2(id: usize) -> bool {
    // Use byte-slices to avoid allocating a Vec<char> and speed comparisons
    // This works because we're only dealing with ASCII digits
    let id_str = id.to_string();
    let bytes = id_str.as_bytes();
    let len = bytes.len();
    let mid = len / 2;

    // First-half equals second-half -> invalid
    // This is to optimize the common case of even-length patterns
    if id_str[..mid] == id_str[mid..] {
        return true;
    }

    // Consider only non-overlapping chunk sizes that evenly divide the string
    for i in 1..=mid {
        if !len.is_multiple_of(i) {
            continue;
        }

        let first_chunk = &bytes[..i];
        let mut all_match = true;
        let mut j = i;
        while j + i <= len {
            if &bytes[j..j + i] != first_chunk {
                all_match = false;
                break;
            }
            j += i;
        }

        if all_match {
            return true;
        }
    }

    false
}

fn solve_part1(puzzle_input: &str) -> usize {
    let invalid_ids_sum: usize = puzzle_input
        .split(COMMA_DELIMITER)
        .filter_map(parse_range)
        .flatten()
        .filter(|&x| is_invalid_id_part1(x))
        .sum();

    println!("Part 1 - Invalid IDS sum is: {invalid_ids_sum}");
    invalid_ids_sum
}

fn solve_part2(puzzle_input: &str) -> usize {
    let invalid_ids_sum: usize = puzzle_input
        .split(COMMA_DELIMITER)
        .filter_map(parse_range)
        .flatten()
        .filter(|&x| is_invalid_id_part2(x))
        .sum();

    println!("Part 2 - Invalid IDS sum is: {invalid_ids_sum}\n");
    invalid_ids_sum
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
        let invalid_ids_sum = solve_part1(puzzle_dummy_input);
        assert_eq!(invalid_ids_sum, 1227775554);
    }

    #[test]
    fn test_part2() {
        let puzzle_dummy_input = get_puzzle_dummy_input_path(PUZZLE_NUMBER);
        let puzzle_dummy_input = str::from_utf8(&puzzle_dummy_input).unwrap_or("");
        let invalid_ids_sum = solve_part2(puzzle_dummy_input);
        assert_eq!(invalid_ids_sum, 4174379265);
    }
}
