use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_7.txt";
pub const TEST: &str = "inputs/test.txt";

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(result: u64, numbers: Vec<u64>) -> Self {
        Self { result, numbers }
    }

    fn is_solvable(&self, functions: Vec<Box<dyn Fn(u64, u64) -> Option<u64>>>) -> bool {
        let _ = functions;
        let mut results = vec![*self.numbers.first().unwrap()];
        let mut numbers = self.numbers.iter().skip(1);
        while let Some(n) = numbers.next() {
            results = functions
                .iter()
                .flat_map(|f| {
                    results.iter().filter_map(|result| {
                        f(*result, *n).and_then(|r| if r <= self.result { Some(r) } else { None })
                    })
                })
                .collect();
        }
        results.contains(&self.result)
    }
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let (result, numbers) = s.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
        Self::new(result, numbers)
    }
}

fn add(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

fn mul(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(b)
}

fn concat(a: u64, b: u64) -> Option<u64> {
    b.checked_ilog10()
        .and_then(|p| 10u64.checked_pow(p + 1))
        .and_then(|p| a.checked_mul(p))
        .and_then(|p| p.checked_add(b))
}

// Elapsed time: 18821 us
// Memory Used: 59.770508 kb
pub fn find_valid_equations_without_concatenation(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    Ok(input
        .lines()
        .filter_map(|l| {
            let equation = Equation::from(l);
            if equation.is_solvable(vec![Box::new(add), Box::new(mul)]) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum())
}

// Elapsed time: 502076 us
// Memory Used: 3611.7861 kb
pub fn find_valid_equations_with_concatenation(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    Ok(input
        .lines()
        .filter_map(|l| {
            let equation = Equation::from(l);
            if equation.is_solvable(vec![Box::new(add), Box::new(mul), Box::new(concat)]) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum())
}
