use std::collections::HashSet;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_12.txt";
pub const TEST: &str = "inputs/test.txt";

const NEIGHBORS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct FarmLand {
    land: Vec<Vec<char>>,
}

impl FarmLand {
    fn new() -> Self {
        FarmLand { land: Vec::new() }
    }

    fn find_fence_price(&self, bulk: bool) -> u64 {
        let mut visited = HashSet::new();
        self.land
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                let mut cost = 0;
                for (j, _c) in row.iter().enumerate() {
                    if visited.insert((i as i64, j as i64)) {
                        let (region_area, region_perimeter, region_sides) = self
                            .find_region_area_perimeter_side((i as i64, j as i64), &mut visited);
                        cost += region_area * (if bulk { region_sides } else { region_perimeter });
                    }
                }
                Some(cost)
            })
            .sum()
    }

    fn find_region_area_perimeter_side(
        &self,
        start: (i64, i64),
        visited: &mut HashSet<(i64, i64)>,
    ) -> (u64, u64, u64) {
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;
        let region_identifier = self.land[start.0 as usize][start.1 as usize];
        let mut queue = vec![start];
        while !queue.is_empty() {
            let (i, j) = queue.pop().unwrap();
            area += 1;
            NEIGHBORS
                .iter()
                .zip(NEIGHBORS.iter().cycle().skip(1))
                .for_each(|(&(di, dj), &(di2, dj2))| {
                    let (ni, nj) = (i + di, j + dj);
                    let (ni2, nj2) = (i + di2, j + dj2);
                    let (ni3, nj3) = (i + di + di2, j + dj + dj2);
                    let is_in_map = self.is_in_map((ni, nj));
                    let is_in_map2 = self.is_in_map((ni2, nj2));
                    let is_in_map3 = self.is_in_map((ni3, nj3));
                    if !is_in_map || self.land[ni as usize][nj as usize] != region_identifier {
                        perimeter += 1;
                        if !is_in_map2 || self.land[ni2 as usize][nj2 as usize] != region_identifier
                        {
                            sides += 1;
                        }
                    } else {
                        if visited.insert((ni, nj)) {
                            queue.push((ni, nj));
                        }
                        if is_in_map2
                            && self.land[ni2 as usize][nj2 as usize] == region_identifier
                            && is_in_map3
                            && self.land[ni3 as usize][nj3 as usize] != region_identifier
                        {
                            sides += 1;
                        }
                    }
                });
        }

        (area, perimeter, sides)
    }

    fn is_in_map(&self, (i, j): (i64, i64)) -> bool {
        (0..self.land.len() as i64).contains(&i)
            && (0..self.land[i as usize].len() as i64).contains(&j)
    }
}

impl From<String> for FarmLand {
    fn from(s: String) -> Self {
        let land = s
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        FarmLand { land }
    }
}

// Elapsed time: 48853 us
// Memory Used: 904.7422 kb
pub fn find_fencing_price(path: &str) -> Result<u64> {
    let farm_land = FarmLand::from(get_input(path)?);
    Ok(farm_land.find_fence_price(false))
}

// Elapsed time: 49986 us
// Memory Used: 904.7422 kb
pub fn find_bulk_fencing_price(path: &str) -> Result<u64> {
    let farm_land = FarmLand::from(get_input(path)?);
    Ok(farm_land.find_fence_price(true))
}
