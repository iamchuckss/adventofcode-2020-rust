use aoc2020::parse;

use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

use std::{collections::HashMap, fs::File};
use std::io::{self, BufRead};

fn find_pair_summing_to(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64)> {
    for datum in data {
        let want = sum - *datum;
        if data.contains(&want) {
            return Some((*datum, want));
        }
    }
    None
}

fn find_triple_summing_to(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    for datum in data {
        let remainder = sum - *datum;
        if let Some((a, b)) = find_pair_summing_to(data, remainder) {
            if a != b && a != *datum && b != *datum {
                return Some((a, b, *datum));
            }
        }
    }
    None
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let inputs: HashSet<i64> = parse(input)?.collect();
    match find_pair_summing_to(&inputs, 2020) {
        Some((a, b)) => {
            println!("{} * {} == {}", a, b, a * b);
        }
        None => println!("pair not found"),
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let inputs: HashSet<i64> = parse(input)?.collect();
    match find_triple_summing_to(&inputs, 2020) {
        Some((a, b, c)) => {
            println!("{} * {} * {} == {}", a, b, c, a * b * c);
        }
        None => println!("triple not found"),
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/*
My Solution
*/

static TARGET: i32 = 2020;

pub fn day_one() {
    // Read entries from input, insert to entries Vec
    let mut entries = Vec::<i32>::new();
    let filename = "./input/day_one_input.txt";
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(num) = line {
                entries.push(num.parse().unwrap());
            }
        }
    }

    // Sort entries ascending order
    entries.sort();

    let part_one_result = part_one_map(&entries);
    let part_two_result = part_two(&entries);

    println!("*****Day 1 results*****");
    println!("Part 1: {}", part_one_result);
    println!("Part 2: {}", part_two_result);
    println!();
}

fn part_one_map(entries: &Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, usize>::new();
    for i in 0..entries.len() {
        let complement = TARGET - entries[i];
        if map.get(&complement).is_some() {
            return entries[i] * complement;
        }
        map.insert(entries[i], i);
    }
    return 0;
}

#[allow(dead_code)]
fn part_one_two_pointers(entries: &Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = entries.len() - 1;
    while left < right {
        let sum = entries[left] + entries[right];
        if sum > TARGET {
            right -= 1;
        } else if sum < TARGET {
            left += 1;
        } else {
            break;
        }
    }

    return entries[left] * entries[right];
}

fn part_two(entries: &Vec<i32>) -> i32 {
    let mut part_two_result = 0;
    for i in  0..entries.len() {
        let mut j = i + 1;
        let mut k = entries.len() - 1;

        while j < k {
            let sum = entries[i] + entries[j] + entries[k];
            if sum > TARGET {
                k -= 1;
            } else if sum < TARGET {
                j += 1;
            } else {
                part_two_result = entries[i] * entries[j] * entries[k];
                break;
            }
        }
    }
    return part_two_result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
