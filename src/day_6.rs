use std::collections::{HashMap, HashSet};

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_6.txt";
pub const TEST: &str = "inputs/test.txt";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Guard {
    position: (i64, i64),
    orientation: Orientation,
}

struct NorthPoleMap {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

struct Walls {
    vertical: HashMap<i64, HashSet<i64>>,
    horizontal: HashMap<i64, HashSet<i64>>,
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

    fn set_position(&mut self, position: (i64, i64)) {
        self.position = position;
    }

    fn turn_right(&mut self) {
        self.orientation = self.orientation.turn_right();
    }
}

impl NorthPoleMap {
    fn new(grid: Vec<Vec<char>>, guard: Guard) -> Self {
        Self { grid, guard }
    }

    fn check_is_cycle(guard: &Guard, walls: &Walls) -> bool {
        let mut seen_positions = HashSet::new();
        let mut guard = guard.clone();
        loop {
            match walls.get_position_before_next_wall(&guard) {
                Some(position) => {
                    guard.set_position(position);
                    guard.turn_right();
                    if !seen_positions.insert(guard.clone()) {
                        return true;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    fn get_guard_locations(&self) -> HashSet<(i64, i64)> {
        let mut locations = HashSet::new();
        let mut guard = self.guard.clone();
        loop {
            locations.insert(guard.get_position());
            let next_position = guard.get_next_position();
            if !self.is_in_map(next_position) {
                break;
            } else if self.grid[next_position.0 as usize][next_position.1 as usize] == '#' {
                guard.turn_right();
            } else {
                guard.r#move();
            }
        }
        locations
    }

    fn get_wall_cycle_locations(&self) -> HashSet<(i64, i64)> {
        let mut walls = Walls::from(self);
        let mut seen_positions = HashSet::new();
        let mut cycle_wall_locations = HashSet::new();
        let mut guard = self.guard.clone();
        loop {
            seen_positions.insert(guard.get_position());
            let next_position = guard.get_next_position();
            if !self.is_in_map(next_position) {
                break;
            } else if self.grid[next_position.0 as usize][next_position.1 as usize] == '#' {
                guard.turn_right();
            } else {
                walls.insert(next_position);
                if !seen_positions.contains(&next_position)
                    && NorthPoleMap::check_is_cycle(&guard, &walls)
                {
                    cycle_wall_locations.insert(next_position);
                }
                walls.remove(next_position);
                guard.r#move();
            }
        }
        cycle_wall_locations.remove(&self.guard.get_position());
        cycle_wall_locations
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

impl Walls {
    fn new() -> Self {
        Self {
            vertical: HashMap::new(),
            horizontal: HashMap::new(),
        }
    }

    fn get_position_before_next_wall(&self, guard: &Guard) -> Option<(i64, i64)> {
        let position = guard.get_position();
        match guard.orientation {
            Orientation::North => self
                .vertical
                .get(&position.1)
                .and_then(|walls| {
                    walls
                        .iter()
                        .filter_map(|wall| {
                            if *wall < position.0 {
                                Some((position.0 - wall, *wall))
                            } else {
                                None
                            }
                        })
                        .min()
                })
                .map(|wall| (wall.1 + 1, position.1)),
            Orientation::East => self
                .horizontal
                .get(&position.0)
                .and_then(|walls| {
                    walls
                        .iter()
                        .filter_map(|wall| {
                            if *wall > position.1 {
                                Some((*wall - position.1, *wall))
                            } else {
                                None
                            }
                        })
                        .min()
                })
                .map(|wall| (position.0, wall.1 - 1)),
            Orientation::South => self
                .vertical
                .get(&position.1)
                .and_then(|walls| {
                    walls
                        .iter()
                        .filter_map(|wall| {
                            if *wall > position.0 {
                                Some((wall - position.0, *wall))
                            } else {
                                None
                            }
                        })
                        .min()
                })
                .map(|wall| (wall.1 - 1, position.1)),
            Orientation::West => self
                .horizontal
                .get(&position.0)
                .and_then(|walls| {
                    walls
                        .iter()
                        .filter_map(|wall| {
                            if *wall < position.1 {
                                Some((position.1 - wall, *wall))
                            } else {
                                None
                            }
                        })
                        .min()
                })
                .map(|wall| (position.0, wall.1 + 1)),
        }
    }

    fn insert(&mut self, wall: (i64, i64)) {
        self.vertical
            .entry(wall.1)
            .or_insert_with(HashSet::new)
            .insert(wall.0);
        self.horizontal
            .entry(wall.0)
            .or_insert_with(HashSet::new)
            .insert(wall.1);
    }

    fn remove(&mut self, wall: (i64, i64)) {
        self.vertical
            .get_mut(&wall.1)
            .map(|walls| walls.remove(&wall.0));
        self.horizontal
            .get_mut(&wall.0)
            .map(|walls| walls.remove(&wall.1));
    }
}

impl From<&NorthPoleMap> for Walls {
    fn from(map: &NorthPoleMap) -> Self {
        let mut walls = Self::new();
        map.grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, c)| {
                if *c == '#' {
                    walls.insert((i as i64, j as i64));
                }
            })
        });
        walls
    }
}

// Elapsed time: 3943 us
// Memory Used: 282.9297 kb
pub fn find_distinct_guard_positions(input: &str) -> Result<usize> {
    let input = get_input(input)?;
    let map = NorthPoleMap::from(input);
    let locations = map.get_guard_locations();
    Ok(locations.len())
}

// Elapsed time: 236772 us
// Memory Used: 375.8711 kb
pub fn find_wall_cycle_locations(input: &str) -> Result<usize> {
    let input = get_input(input)?;
    let map = NorthPoleMap::from(input);
    let wall_cycle_locations = map.get_wall_cycle_locations();
    Ok(wall_cycle_locations.len())
}
