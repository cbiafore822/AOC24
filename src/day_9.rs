use std::{fmt::Debug, iter};

use crate::get_input;
use anyhow::Result;

pub const INPUT: &str = "inputs/day_9.txt";
pub const TEST: &str = "inputs/test.txt";

#[derive(Clone)]
enum Data {
    File(u64, u64),
    Empty(u64),
}

struct FileSystem {
    data: Vec<Option<u64>>,
}

struct FileSystemImproved {
    data: Vec<Data>,
}

impl Data {
    fn get_size(&self) -> u64 {
        match self {
            Data::File(_, size) => *size,
            Data::Empty(size) => *size,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Data::Empty(_))
    }
    fn is_file(&self) -> bool {
        matches!(self, Data::File(_, _))
    }

    fn fid(&self) -> u64 {
        match self {
            Data::File(i, _) => *i,
            Data::Empty(_) => panic!("Empty file"),
        }
    }

    fn fid_or(&self, default: u64) -> u64 {
        match self {
            Data::File(i, _) => *i,
            Data::Empty(_) => default,
        }
    }
}

impl FileSystem {
    pub fn new(data: Vec<Option<u64>>) -> Self {
        Self { data }
    }

    fn compress_data(&mut self) {
        let mut i = self.data.iter().position(|f| f.is_none()).unwrap();
        let mut j = self.data.len() - self.data.iter().rev().position(|f| f.is_some()).unwrap() - 1;
        while i < j {
            if self.data[j].is_none() {
                j -= 1;
            } else if self.data[i].is_some() {
                i += 1;
            } else {
                self.data.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
    }

    fn calculate_checksum(&self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .map(|(i, f)| i as u64 * f.unwrap_or(0))
            .sum()
    }
}

impl From<String> for FileSystem {
    fn from(s: String) -> Self {
        let data = s
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                let size = c.to_digit(10).unwrap();
                match i % 2 {
                    0 => vec![Some(i as u64 / 2); size as usize],
                    1 => vec![None; size as usize],
                    _ => unreachable!(),
                }
            })
            .collect();
        Self { data }
    }
}

impl Debug for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|v| {
            write!(f, "{}", v.map_or(String::from("."), |v| v.to_string())).unwrap();
        });
        Ok(())
    }
}

impl FileSystemImproved {
    pub fn new(data: Vec<Data>) -> Self {
        Self { data }
    }

    fn compress_data(&mut self) {
        let mut i = self.data.iter().position(|f| f.is_empty()).unwrap();
        let mut j = self.data.len() - self.data.iter().rev().position(|f| f.is_file()).unwrap() - 1;
        while i < j {
            if self.data[j].is_empty() {
                j -= 1;
            } else if self.data[i].is_file() {
                i += 1;
            } else {
                let file_size = self.data[j].get_size();
                match self
                    .data
                    .iter()
                    .enumerate()
                    .skip(i)
                    .position(|(ind, f)| ind < j && f.is_empty() && f.get_size() >= file_size)
                {
                    Some(k) => {
                        let replace_ind = i + k;
                        let empty_size = self.data[replace_ind].get_size();
                        match file_size.cmp(&empty_size) {
                            std::cmp::Ordering::Less => {
                                let temp = self.data[j].clone();
                                self.data[j] = Data::Empty(file_size);
                                self.data.splice(
                                    replace_ind..replace_ind + 1,
                                    [
                                        Data::File(temp.fid(), file_size),
                                        Data::Empty(empty_size - file_size),
                                    ],
                                );
                            }
                            std::cmp::Ordering::Equal => {
                                self.data.swap(replace_ind, j);
                                j -= 1;
                            }
                            std::cmp::Ordering::Greater => unreachable!(),
                        }
                    }
                    None => j -= 1,
                }
            }
        }
    }

    fn calculate_checksum(&self) -> u64 {
        self.data
            .iter()
            .fold((0, 0), |(total, ind), f| {
                let size = f.get_size();
                let fid = f.fid_or(0);
                (
                    total + (ind..ind + size).map(|i| i).sum::<u64>() * fid,
                    ind + size,
                )
            })
            .0
    }
}

impl From<String> for FileSystemImproved {
    fn from(s: String) -> Self {
        let data = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let size = c.to_digit(10).unwrap();
                match i % 2 {
                    0 => Data::File(i as u64 / 2, size as u64),
                    1 => Data::Empty(size as u64),
                    _ => unreachable!(),
                }
            })
            .collect();
        Self { data }
    }
}

impl Debug for FileSystemImproved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|v| match v {
            Data::File(fid, size) => write!(
                f,
                "{}",
                iter::repeat(fid.to_string())
                    .take(*size as usize)
                    .collect::<String>()
            )
            .unwrap(),
            Data::Empty(size) => write!(
                f,
                "{}",
                iter::repeat(".").take(*size as usize).collect::<String>()
            )
            .unwrap(),
        });
        Ok(())
    }
}

// Elapsed time: 24319 us
// Memory Used: 2711.5068 kb
pub fn compress_filesystem(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let mut filesystem = FileSystem::from(input);
    filesystem.compress_data();
    Ok(filesystem.calculate_checksum())
}

// Elapsed time: 8176778 us
// Memory Used: 1410.3672 kb
pub fn compress_filesystem_improved(path: &str) -> Result<u64> {
    let input = get_input(path)?;
    let mut filesystem = FileSystemImproved::from(input);
    filesystem.compress_data();
    Ok(filesystem.calculate_checksum())
}
