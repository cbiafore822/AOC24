use std::collections::HashSet;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_6.txt";
pub const TEST: &str = "inputs/test.txt";

struct Guard {
    position: (i64, i64),
    orientation: Orientation,
}

struct NorthPoleMap {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

enum Orientation {
    North,
    East,
    South,
    West,
}

impl Guard {
    fn new(position: (i64, i64), orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
        }
    }

    fn get_position(&self) -> (i64, i64) {
        self.position
    }

    fn get_next_position(&self) -> (i64, i64) {
        match self.orientation {
            Orientation::North => (self.position.0 - 1, self.position.1),
            Orientation::East => (self.position.0, self.position.1 + 1),
            Orientation::South => (self.position.0 + 1, self.position.1),
            Orientation::West => (self.position.0, self.position.1 - 1),
        }
    }

    fn r#move(&mut self) {
        match self.orientation {
            Orientation::North => self.position.0 -= 1,
            Orientation::East => self.position.1 += 1,
            Orientation::South => self.position.0 += 1,
            Orientation::West => self.position.1 -= 1,
        }
    }

    fn turn_right(&mut self) {
        self.orientation = self.orientation.turn_right();
    }
}

impl NorthPoleMap {
    fn new(grid: Vec<Vec<char>>, guard: Guard) -> Self {
        Self { grid, guard }
    }

    fn get_guard_locations(&mut self) -> HashSet<(i64, i64)> {
        let mut locations = HashSet::new();
        loop {
            locations.insert(self.guard.get_position());
            let next_position = self.guard.get_next_position();
            if !self.is_in_map(next_position) {
                break;
            } else if self.grid[next_position.0 as usize][next_position.1 as usize] == '#' {
                self.guard.turn_right();
            } else {
                self.guard.r#move();
            }
        }
        locations
    }

    fn is_in_map(&self, position: (i64, i64)) -> bool {
        (0..self.grid.len()).contains(&(position.0 as usize))
            && (0..self.grid[position.0 as usize].len()).contains(&(position.1 as usize))
    }
}

impl From<String> for NorthPoleMap {
    fn from(input: String) -> Self {
        let obstacles = input.lines().map(|line| line.chars().collect()).collect();
        let guard = input
            .lines()
            .enumerate()
            .find_map(|(i, line)| {
                line.chars().enumerate().find_map(|(j, c)| match c {
                    '^' => Some(Guard::new((i as i64, j as i64), Orientation::North)),
                    '>' => Some(Guard::new((i as i64, j as i64), Orientation::East)),
                    'v' => Some(Guard::new((i as i64, j as i64), Orientation::South)),
                    '<' => Some(Guard::new((i as i64, j as i64), Orientation::West)),
                    _ => None,
                })
            })
            .unwrap();
        Self::new(obstacles, guard)
    }
}

impl Orientation {
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

// Elapsed time: 3960 us
// Memory Used: 282.9297 kb
pub fn find_distinct_guard_positions(input: &str) -> Result<usize> {
    let input = get_input(input)?;
    let mut map = NorthPoleMap::from(input);
    let locations = map.get_guard_locations();
    Ok(locations.len())
}
