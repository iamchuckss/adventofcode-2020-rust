use aoc2020::{parse};
use itertools::Itertools;

use std::{collections::{HashMap}, path::Path};
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let adapters = parse::<u32>(input)?.sorted();

    let mut diff_count: HashMap<u32, u32> = HashMap::new();
    let mut prev = 0;
    for adapter in adapters {
        let diff = adapter - prev;
        let val = diff_count.entry(diff).or_insert(0);
        *val += 1;
        prev = adapter;
    }

    // Account for in-built adapter (diff of 3)
    let val = diff_count.entry(3).or_insert(0);
    *val += 1;

    let diff_1 = diff_count.get(&1).unwrap();
    let diff_3 = diff_count.get(&3).unwrap();
    println!("{:#?}", diff_1 * diff_3);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut adapters = parse::<u32>(input)?
        .sorted()
        .collect::<Vec<u32>>();

    let inbuilt = adapters.iter().max().unwrap() + 3;
    adapters.push(inbuilt);

    let mut cache = HashMap::new();
    let total_ways = count_ways(&adapters, 0, 0, &mut cache);

    println!("{}", total_ways);
    Ok(())
}

fn count_ways(adapters: &Vec<u32>, idx: usize, prev: u32, cache: &mut HashMap<(usize, u32), u64>) -> u64 {
    if idx >= adapters.len() { return 1; }
    if (adapters[idx] - prev) > 3 { return 0; }
    if cache.contains_key(&(idx, prev)) { return *cache.get(&(idx, prev)).unwrap(); }

    // println!("{} {}", idx, prev);
    let res = count_ways(adapters, idx + 1, adapters[idx], cache) + 
    count_ways(adapters, idx + 1, prev, cache);

    cache.insert((idx, prev), res);
    return res;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}
