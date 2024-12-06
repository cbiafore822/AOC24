use std::collections::{HashMap, HashSet};

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_5.txt";
pub const TEST: &str = "inputs/test.txt";

struct SafetyManual {
    page_ordering: HashMap<u64, HashSet<u64>>,
    updates: Vec<Vec<u64>>,
}

impl SafetyManual {
    fn new(page_ordering: HashMap<u64, HashSet<u64>>, updates: Vec<Vec<u64>>) -> Self {
        Self {
            page_ordering,
            updates,
        }
    }

    fn calculate_valid_update_sums(&self) -> u64 {
        self.updates
            .iter()
            .filter_map(|update| {
                let mut seen = HashSet::new();
                if update.iter().all(|page| {
                    if let Some(rules) = self.page_ordering.get(page) {
                        if seen.intersection(rules).count() == 0 {
                            seen.insert(*page);
                            return true;
                        } else {
                            return false;
                        }
                    }
                    seen.insert(*page);
                    true
                }) {
                    Some(update[update.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn calculate_incorrectly_update_sums(&self) -> u64 {
        self.updates
            .iter()
            .filter_map(|update| {
                let mut curr_update = update.clone();
                let mut swaps = 0;
                loop {
                    let mut seen = HashMap::new();
                    let valid = curr_update.clone().iter().enumerate().all(|(ind, page)| {
                        if let Some(rules) = self.page_ordering.get(&page) {
                            let vals = seen.keys().cloned().collect::<HashSet<_>>();
                            let invalid_pages = vals.intersection(rules).collect::<HashSet<_>>();
                            if invalid_pages.len() == 0 {
                                seen.insert(*page, ind);
                                return true;
                            } else {
                                let first_invalid_page =
                                    invalid_pages.iter().map(|page| seen[page]).min().unwrap();
                                curr_update.swap(ind, first_invalid_page);
                                swaps += 1;
                                return false;
                            }
                        }
                        seen.insert(*page, ind);
                        true
                    });
                    match (valid, swaps) {
                        (true, 0) => return None,
                        (true, _) => return Some(curr_update[curr_update.len() / 2]),
                        (false, _) => continue,
                    }
                }
            })
            .sum()
    }
}

impl From<String> for SafetyManual {
    fn from(s: String) -> Self {
        let (ordering_rules, updates) = s.split_once("\n\n").unwrap();
        let page_ordering = ordering_rules
            .lines()
            .fold(HashMap::new(), |mut map, rule| {
                let (page, rule) = rule.split_once("|").unwrap();
                let page = page.parse().unwrap();
                let rule = rule.parse().unwrap();
                map.entry(page).or_insert_with(HashSet::new).insert(rule);
                map
            });
        let updates = updates
            .lines()
            .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
            .collect();
        SafetyManual::new(page_ordering, updates)
    }
}

// Elapsed time: 3987 us
// Memory Used: 75.66113 kb
pub fn calculate_correctly_ordered_updates(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let manual = SafetyManual::from(input);
    Ok(manual.calculate_valid_update_sums())
}

// Elapsed time: 337885 us
// Memory Used: 75.66113 kb
pub fn calculate_incorrectly_ordered_updates(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let manual = SafetyManual::from(input);
    Ok(manual.calculate_incorrectly_update_sums())
}
