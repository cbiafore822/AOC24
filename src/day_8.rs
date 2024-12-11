use std::collections::{HashMap, HashSet};

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_8.txt";
pub const TEST: &str = "inputs/test.txt";

struct Antennas {
    grid: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<(i64, i64)>>,
}

impl Antennas {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let antennas = grid
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, row)| {
                row.iter().enumerate().for_each(|(x, frequency)| {
                    if frequency.is_alphanumeric() {
                        acc.entry(*frequency)
                            .or_insert_with(Vec::new)
                            .push((x as i64, y as i64));
                    }
                });
                acc
            });
        Self { grid, antennas }
    }

    fn find_antinodes(&self, jumps: Option<u64>, starting_jump: u64) -> HashSet<(i64, i64)> {
        let antenna_pairs = self.get_antenna_pairs();

        antenna_pairs
            .iter()
            .flat_map(|(_frequency, pairs)| {
                pairs.iter().flat_map(|(a, b)| {
                    let mut antinodes = Vec::new();
                    let mut curr_jumps = starting_jump;
                    let dx = b.0 - a.0;
                    let dy = b.1 - a.1;
                    while jumps.map_or(true, |j| curr_jumps <= j) {
                        let antinode_1 =
                            (b.0 + dx * curr_jumps as i64, b.1 + dy * curr_jumps as i64);
                        let antinode_2 =
                            (a.0 - dx * curr_jumps as i64, a.1 - dy * curr_jumps as i64);
                        match (self.is_in_grid(antinode_1), self.is_in_grid(antinode_2)) {
                            (true, true) => {
                                antinodes.push(antinode_1);
                                antinodes.push(antinode_2);
                            }
                            (true, false) => antinodes.push(antinode_1),
                            (false, true) => antinodes.push(antinode_2),
                            _ => break,
                        }
                        curr_jumps += 1;
                    }
                    antinodes
                })
            })
            .collect()
    }

    fn get_antenna_pairs(&self) -> HashMap<char, Vec<((i64, i64), (i64, i64))>> {
        self.antennas
            .iter()
            .map(|(frequency, antennas)| {
                (
                    *frequency,
                    antennas
                        .iter()
                        .flat_map(|a| antennas.iter().map(move |b| (*a, *b)))
                        .filter(|(a, b)| a != b)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    fn is_in_grid(&self, node: (i64, i64)) -> bool {
        (0..self.grid.len() as i64).contains(&node.1)
            && (0..self.grid[0].len() as i64).contains(&node.0)
    }
}

impl From<String> for Antennas {
    fn from(s: String) -> Self {
        let grid = s.lines().map(|l| l.chars().collect()).collect();
        Self::new(grid)
    }
}

// Elapsed time: 620 us
// Memory Used: 53.398438 kb
pub fn find_all_antinodes(path: &str) -> Result<usize> {
    let input = get_input(path)?;
    let antennas = Antennas::from(input);
    Ok(antennas.find_antinodes(Some(1), 1).len())
}

// Elapsed time: 1462 us
// Memory Used: 66.21094 kb
pub fn find_all_extended_antinodes(path: &str) -> Result<usize> {
    let input = get_input(path)?;
    let antennas = Antennas::from(input);
    Ok(antennas.find_antinodes(None, 0).len())
}
