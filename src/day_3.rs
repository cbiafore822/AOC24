use crate::get_input;
use anyhow::Result;
use regex::Regex;

pub const INPUT: &str = "inputs/day_3.txt";
pub const TEST: &str = "inputs/test.txt";

// Elapsed time: 5582 us
// Memory Used: 354.2256 kb
pub fn calculate_corrupted_mul_instructions(path: &str) -> Result<i64> {
    let input = get_input(path)?;
    let instructions_regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    Ok(instructions_regex
        .captures_iter(&input)
        .map(|cap| {
            let first = cap["first"].parse::<i64>().unwrap();
            let second = cap["second"].parse::<i64>().unwrap();
            first * second
        })
        .sum())
}

// Elapsed time: 7897 us
// Memory Used: 359.7959 kb
pub fn calculate_corrupted_mul_do_instructions(path: &str) -> Result<i64> {
    let input = get_input(path)?;
    let instructions_regex =
        Regex::new(r"do\(\)|don\'t\(\)|mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut ignore = false;
    Ok(instructions_regex
        .captures_iter(&input)
        .map(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                ignore = false;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                ignore = true;
            } else if !ignore {
                let first = cap["first"].parse::<i64>().unwrap();
                let second = cap["second"].parse::<i64>().unwrap();
                return first * second;
            }
            0
        })
        .sum())
}
