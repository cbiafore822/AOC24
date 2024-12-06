use std::collections::HashSet;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_4.txt";
pub const TEST: &str = "inputs/test.txt";

pub const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
pub const DIAGNOL_NEIGHBORS: [(i64, i64); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

pub const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
pub const MAS: [char; 2] = ['M', 'S'];

pub struct WordSearch {
    pub grid: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }

    pub fn find_xmas_count(&self) -> u64 {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, c)| {
                    if *c != 'X' {
                        return 0;
                    }
                    NEIGHBORS
                        .iter()
                        .map(move |direction| {
                            if self.check_xmas_match(&i, &j, *direction) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum()
                })
            })
            .sum()
    }

    pub fn find_xmas_cross_count(&self) -> u64 {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, c)| {
                    if *c != 'A' {
                        0
                    } else if self.check_xmas_cross_match(&i, &j) {
                        1
                    } else {
                        0
                    }
                })
            })
            .sum()
    }

    fn check_xmas_match(&self, i: &usize, j: &usize, direction: (i64, i64)) -> bool {
        XMAS.iter().enumerate().all(|(ind, c)| {
            let new_i = *i as i64 + direction.0 * ind as i64;
            let new_j = *j as i64 + direction.1 * ind as i64;
            if (0..self.grid.len()).contains(&(new_i as usize))
                && (0..self.grid[new_i as usize].len()).contains(&(new_j as usize))
            {
                self.grid[new_i as usize][new_j as usize] == *c
            } else {
                false
            }
        })
    }

    fn check_xmas_cross_match(&self, i: &usize, j: &usize) -> bool {
        DIAGNOL_NEIGHBORS.chunks(2).all(|corners| {
            let letters = corners
                .iter()
                .filter_map(|(d_i, d_j)| {
                    let new_i = *i as i64 + d_i;
                    let new_j = *j as i64 + d_j;
                    if (0..self.grid.len()).contains(&(new_i as usize))
                        && (0..self.grid[new_i as usize].len()).contains(&(new_j as usize))
                    {
                        Some(self.grid[new_i as usize][new_j as usize])
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();
            MAS.iter().all(|c| letters.contains(c))
        })
    }
}

impl From<String> for WordSearch {
    fn from(input: String) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }
}

// Elapsed time: 7550 us
// Memory Used: 108.143555 kb
pub fn find_xmas_wordsearch(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let word_search = WordSearch::from(input);
    Ok(word_search.find_xmas_count())
}

// Elapsed time: 10769 us
// Memory Used: 108.143555 kb
pub fn find_xmas_cross_wordsearch(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let word_search = WordSearch::from(input);
    Ok(word_search.find_xmas_cross_count())
}
