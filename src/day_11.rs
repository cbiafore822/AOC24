use std::collections::HashMap;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_11.txt";
pub const TEST: &str = "inputs/test.txt";

struct Stones {
    stones: Vec<u64>,
}

impl Stones {
    fn new() -> Self {
        Stones { stones: Vec::new() }
    }

    fn count_stones_after_blinks(&self, n: u64) -> u64 {
        let mut cache = HashMap::new();
        self.stones
            .iter()
            .map(|stone| self.blink_stone(stone, n, &mut cache))
            .sum()
    }

    fn blink_stone(&self, stone: &u64, n: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
        if n == 0 {
            return 1;
        }
        if let Some(&count) = cache.get(&(*stone, n)) {
            return count;
        }
        let count = if *stone == 0 {
            self.blink_stone(&1, n - 1, cache)
        } else if stone.ilog10() % 2 == 1 {
            let half = 10u64.pow((stone.ilog10() + 1) / 2);
            self.blink_stone(&(stone / half), n - 1, cache)
                + self.blink_stone(&(stone % half), n - 1, cache)
        } else {
            self.blink_stone(&(stone * 2024), n - 1, cache)
        };
        cache.insert((*stone, n), count);
        count
    }
}

impl From<String> for Stones {
    fn from(s: String) -> Self {
        let stones = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Stones { stones }
    }
}

// Part 1
// Elapsed time: 3663 us
// Memory Used: 153.92969 kb
//
// Part 2
// Elapsed time: 169056 us
// Memory Used: 9603.93 kb
pub fn get_total_stones_after_blinks(path: &str, n: u64) -> Result<u64> {
    let input = get_input(path)?;
    let stones = Stones::from(input);
    Ok(stones.count_stones_after_blinks(n))
}
