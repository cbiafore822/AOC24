#![allow(dead_code)]

use chrono::prelude::Local;
use peak_alloc::PeakAlloc;
use std::{
    fs::File,
    io::{Read, Result},
};

mod day_1;
mod day_2;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let start = Local::now();
    let res = day_2::get_safe_reports_with_dampaner(day_2::INPUT).unwrap();
    let duration = (Local::now() - start).num_microseconds().unwrap();
    println!("Result: {}", res);
    println!("Elapsed time: {} us", duration);
    println!("Memory Used: {} kb", PEAK_ALLOC.peak_usage_as_kb());
}

pub fn get_input(path: &str) -> Result<String> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}
