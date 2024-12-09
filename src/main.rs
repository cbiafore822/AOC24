#![allow(dead_code)]

use chrono::prelude::Local;
use peak_alloc::PeakAlloc;
use std::{
    fs::File,
    io::{Read, Result},
};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let start = Local::now();
    let res = day_6::find_distinct_guard_positions(day_6::INPUT).unwrap();
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
