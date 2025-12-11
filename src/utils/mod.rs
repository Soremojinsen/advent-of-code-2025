use std::{fs, path::Path};

const PUZZLE_INPUT_PREFIX: &str = "puzzles_inputs/day";
const PUZZLE_INPUT_SUFFIX: &str = ".txt";

pub fn get_puzzle_input_path(day: usize) -> Vec<u8> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let puzzle_input_path =
        Path::new(manifest_dir).join(format!("{PUZZLE_INPUT_PREFIX}{day}{PUZZLE_INPUT_SUFFIX}"));
    fs::read(puzzle_input_path)
        .unwrap_or_else(|_| panic!("Unexpected error reading puzzle input for day {}", day))
}
