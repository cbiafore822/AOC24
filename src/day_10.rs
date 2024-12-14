use std::collections::HashSet;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_10.txt";
pub const TEST: &str = "inputs/test.txt";

const NEIGHBORS: [(i64, i64); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

struct Mountain {
    peaks: Vec<Vec<u8>>,
}

impl Mountain {
    fn new() -> Self {
        Mountain { peaks: Vec::new() }
    }

    fn find_total_trails(&self) -> u64 {
        self.peaks
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, &p)| {
                    if p == 0 {
                        self.find_reachable_peaks((i as i64, j as i64))
                            .iter()
                            .collect::<HashSet<_>>()
                            .len() as u64
                    } else {
                        0
                    }
                })
            })
            .sum()
    }

    fn find_total_distinct_trails(&self) -> u64 {
        self.peaks
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, &p)| {
                    if p == 0 {
                        self.find_reachable_peaks((i as i64, j as i64)).len() as u64
                    } else {
                        0
                    }
                })
            })
            .sum()
    }

    fn find_reachable_peaks(&self, (i, j): (i64, i64)) -> Vec<(i64, i64)> {
        if self.peaks[i as usize][j as usize] == 9 {
            return Vec::from([(i, j)]);
        }
        let curr_elevation = self.peaks[i as usize][j as usize];
        NEIGHBORS
            .iter()
            .flat_map(|(di, dj)| {
                let ni = i + di;
                let nj = j + dj;
                if self.is_in_mountain((ni, nj))
                    && self.peaks[ni as usize][nj as usize] == curr_elevation + 1
                {
                    self.find_reachable_peaks((ni, nj))
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    fn is_in_mountain(&self, (i, j): (i64, i64)) -> bool {
        (0..self.peaks.len() as i64).contains(&i) && (0..self.peaks[0].len() as i64).contains(&j)
    }
}

impl From<String> for Mountain {
    fn from(value: String) -> Self {
        let peaks = value
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect()
            })
            .collect();
        Mountain { peaks }
    }
}

// Elapsed time: 14248 us
// Memory Used: 11.473633 kb
pub fn find_total_trailheads(path: &str) -> Result<u64> {
    let mountain = Mountain::from(get_input(path)?);
    Ok(mountain.find_total_trails())
}

// Elapsed time: 12470 us
// Memory Used: 11.473633 kb
pub fn find_total_distinct_trailheads(path: &str) -> Result<u64> {
    let mountain = Mountain::from(get_input(path)?);
    Ok(mountain.find_total_distinct_trails())
}
