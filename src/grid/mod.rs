use std::convert::TryFrom;
use std::ops::Not;

const DEFAULT_COLUMN_VALUE: usize = 0;
const DIRECTIONS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

// 2D row-major grid struct
#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T> Grid<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl Grid<u8> {
    pub fn parse(raw_data: &str) -> Result<Self, &'static str> {
        raw_data
            .split_whitespace()
            .map(|row| row.as_bytes().to_vec())
            .collect::<Vec<_>>()
            .try_into()
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }

    fn is_valid_coord(&self, rows: usize, columns: usize) -> bool {
        rows < self.rows && columns < self.columns
    }

    fn get_index(&self, rows: usize, columns: usize) -> usize {
        rows * self.columns + columns
    }

    pub fn get(&self, rows: usize, columns: usize) -> Option<&u8> {
        if self.is_valid_coord(rows, columns).not() {
            return None;
        }
        self.data.get(self.get_index(rows, columns))
    }

    pub fn get_mut(&mut self, rows: usize, columns: usize) -> Option<&mut u8> {
        if self.is_valid_coord(rows, columns).not() {
            return None;
        }
        let index = self.get_index(rows, columns);
        self.data.get_mut(index)
    }

    pub fn find_neighbors(&self, rows: usize, columns: usize) -> Vec<u8> {
        DIRECTIONS
            .into_iter()
            .filter_map(|[dr, dc]| {
                if let (Some(new_rows), Some(new_columns)) =
                    (rows.checked_add_signed(dr), columns.checked_add_signed(dc))
                {
                    self.get(new_rows, new_columns).copied()
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = &'static str;

    fn try_from(two_directional_array: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = two_directional_array.len();
        let columns = two_directional_array
            .first()
            .map_or(DEFAULT_COLUMN_VALUE, |r| r.len());

        if !two_directional_array.iter().all(|r| r.len() == columns) {
            return Err("All rows must have the same length");
        }

        let data = two_directional_array.into_iter().flatten().collect();

        Ok(Grid {
            data,
            rows,
            columns,
        })
    }
}
