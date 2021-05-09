use aoc2020::{parse};

use std::{collections::{HashSet}, path::Path};
use thiserror::Error;

const PREAMPLE_SIZE: usize = 25;

pub fn part1(input: &Path) -> Result<(), Error> {
    let input: Vec<i64> = parse(input)?
        .map(|item: String| item.parse::<i64>().unwrap())
        .collect();

    let first_invalid_preample = get_invalid_preample(&input, PREAMPLE_SIZE);
    println!("{}", first_invalid_preample.unwrap());
    Ok(())
}

fn get_invalid_preample(input: &Vec<i64>, preample_size: usize) -> Option<i64> {
    let mut i = 0;
    let mut j = preample_size;

    while j < input.len() {
        if two_sum(&input[i..j], input[j]).is_none() {
            return Some(input[j]);       
        }
        i += 1;
        j += 1;
    }
    None
}

fn two_sum(nums: &[i64], target: i64) -> Option<i64> {
    let mut set: HashSet<i64> = HashSet::new();

    for num in nums {
        let complement = target - num;
        if set.contains(&complement) {
            return Some(*num);
        }
        set.insert(*num);
    }
    None
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input: Vec<i64> = parse(input)?
        .map(|item: String| item.parse::<i64>().unwrap())
        .collect();

    let first_invalid_preample = get_invalid_preample(&input, PREAMPLE_SIZE).unwrap();
    let contiguous_sum = get_contiguous_sum(&input, first_invalid_preample).unwrap();
    
    let max = contiguous_sum.iter().max().unwrap();
    let min = contiguous_sum.iter().min().unwrap();
    println!("{:?}", max + min);
    Ok(())
}

fn get_contiguous_sum(input: &Vec<i64>, target: i64) -> Option<Vec<i64>> {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while j < input.len() {
        sum += input[j];
        j += 1;

        while sum >= target {
            if sum == target && j - i >= 2{
                let contiguous_sum = &input[i..j];
                return Some(contiguous_sum.iter().cloned().collect::<Vec<i64>>());
            }
            sum -= input[i];
            i += 1;
        }
    }
    None
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}
