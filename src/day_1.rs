use std::collections::HashMap;

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_1.txt";
pub const TEST: &str = "inputs/test.txt";

struct FrequencyList {
    frequencies: HashMap<i64, i64>,
}

struct LocationList {
    locations: Vec<i64>,
}

impl FrequencyList {
    fn new() -> FrequencyList {
        FrequencyList {
            frequencies: HashMap::new(),
        }
    }

    fn get(&self, key: i64) -> i64 {
        *self.frequencies.get(&key).unwrap_or(&0)
    }
}

impl From<Vec<i64>> for FrequencyList {
    fn from(v: Vec<i64>) -> FrequencyList {
        let mut freqs = FrequencyList::new();
        v.iter().for_each(|n| {
            let count = freqs.frequencies.entry(*n).or_insert(0);
            *count += 1;
        });
        freqs
    }
}

impl LocationList {
    fn new() -> LocationList {
        LocationList {
            locations: Vec::new(),
        }
    }

    fn find_distance(&self, other: &LocationList) -> u64 {
        self.locations
            .iter()
            .zip(other.locations.iter())
            .fold(0, |acc, (a, b)| acc + (a - b).abs() as u64)
    }

    fn find_similarity(&self, frequencies: &FrequencyList) -> u64 {
        self.locations.iter().fold(0, |acc, loc| {
            acc + *loc as u64 * frequencies.get(*loc) as u64
        })
    }
}

impl From<Vec<i64>> for LocationList {
    fn from(v: Vec<i64>) -> LocationList {
        LocationList { locations: v }
    }
}

impl From<&mut Vec<i64>> for LocationList {
    fn from(v: &mut Vec<i64>) -> LocationList {
        v.sort();
        LocationList {
            locations: v.to_vec(),
        }
    }
}

// Elapsed time: 916 us
// Memory Used: 56.944336 kb
pub fn list_distance(input_path: &str) -> Result<u64> {
    let input = get_input(input_path)?;
    let locs = input
        .lines()
        .filter_map(|l| {
            l.split_once("   ")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect::<Vec<(i64, i64)>>();

    let loc1 = LocationList::from(locs.iter().map(|(a, _)| *a).collect::<Vec<i64>>().as_mut());
    let loc2 = LocationList::from(locs.iter().map(|(_, b)| *b).collect::<Vec<i64>>().as_mut());
    Ok(loc1.find_distance(&loc2))
}

// Elapsed time: 1213 us
// Memory Used: 74.663086 kb
pub fn get_similarity_score(input_path: &str) -> Result<u64> {
    let input = get_input(input_path)?;
    let nums = input
        .lines()
        .filter_map(|l| {
            l.split_once("   ")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect::<Vec<(i64, i64)>>();

    let locs = LocationList::from(nums.iter().map(|(a, _)| *a).collect::<Vec<i64>>().as_mut());
    let freqs = FrequencyList::from(nums.iter().map(|(_, b)| *b).collect::<Vec<i64>>());
    Ok(locs.find_similarity(&freqs))
}
