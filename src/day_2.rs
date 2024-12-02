use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_2.txt";
pub const TEST: &str = "inputs/test.txt";

struct Levels {
    levels: Vec<Vec<i64>>,
}

impl Levels {
    fn new() -> Levels {
        Levels { levels: Vec::new() }
    }

    fn find_safe_reports(&self, with_dampaner: bool) -> u64 {
        self.levels.iter().fold(0, |acc, level| {
            let is_safe = Self::is_safe(level);
            if is_safe {
                acc + 1
            } else if with_dampaner {
                acc + level.iter().enumerate().any(|(i, _)| {
                    let mut new_level = level.clone();
                    new_level.remove(i);
                    Self::is_safe(&new_level)
                }) as u64
            } else {
                acc
            }
        })
    }

    fn is_safe(level: &Vec<i64>) -> bool {
        let differences = level.windows(2).map(|w| w[1] - w[0]);
        let is_consistent =
            differences.clone().all(|d| d < 0) || differences.clone().all(|d| d > 0);
        let is_within_step = differences.clone().all(|d| 1 <= d.abs() && d.abs() <= 3);
        is_consistent && is_within_step
    }
}

impl From<String> for Levels {
    fn from(s: String) -> Levels {
        let levels = s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        Levels { levels }
    }
}

impl From<Vec<Vec<i64>>> for Levels {
    fn from(v: Vec<Vec<i64>>) -> Levels {
        Levels { levels: v }
    }
}

// Elapsed time: 1739 us
// Memory Used: 109.03613 kb
pub fn get_safe_reports(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let levels = Levels::from(input);
    Ok(levels.find_safe_reports(false))
}

// Elapsed time: 2996 us
// Memory Used: 109.03613 kb
pub fn get_safe_reports_with_dampaner(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let levels = Levels::from(input);
    Ok(levels.find_safe_reports(true))
}
