use aoc2020::input::parse_newline_sep;

use std::{collections::{HashMap, HashSet}, path::Path};
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let total_count: usize = parse_newline_sep::<String>(input)?
    .map(|s| {
        s.trim()
         .chars()
         .filter(|c| !c.is_whitespace())
         .collect::<HashSet<char>>()
         .len()
    })
    .sum();

    println!("{}", total_count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let total_count: usize = parse_newline_sep::<String>(input)?
    .map(|s| {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        let total_lines = s.trim().chars().filter(|c| c == &'\n').collect::<String>().len() as i32 + 1;
        
        s.trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .for_each(|c| {
                let count = char_count.entry(c).or_insert(0);
                *count += 1
            });

        char_count.iter()
            .filter(|&(_, v)| *v == total_lines as i32)
            .count()
    })
    .sum();

    println!("{}", total_count);

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}


/* Alternative Solution

*/