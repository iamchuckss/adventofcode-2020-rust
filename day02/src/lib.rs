use aoc2020::parse;

use std::path::Path;
use thiserror::Error;
use std::{collections::HashMap};
use itertools::Itertools;

pub fn part1(input: &Path) -> Result<(), Error> {
    let num_valid = parse(input)?
        .map(|line: String| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            if tokens.len() != 3 {
                return false;
            }

            let (first, second, password) = (tokens[0], tokens[1], tokens[2]);
            let (lo, hi) = parse_lo_hi(&first.to_string());            
            let target_char = parse_target_char(&second.to_string());

            string_contains_n_char(password, &target_char, lo, hi)            
        })
        .filter(|&item| item )
        .count();
    
    println!("{}", num_valid);
    Ok(())
}

fn parse_lo_hi(input: &String) -> (i32, i32) {
    let (lo, hi): (i32, i32) = input.split('-')
        .map(|string| string.to_string().parse().unwrap())
        .collect_tuple()
        .unwrap();
    (lo, hi)
}

fn parse_target_char(input: &String) -> char {
    input.split(':').take(1).next().unwrap().chars().next().unwrap()
}

fn string_contains_n_char(input: &str, target_char: &char, lo: i32, hi: i32) -> bool {
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for c in input.chars() {
        let old_count = char_count.get(&c).unwrap_or(&0);
        let new_count = *old_count + 1;
        char_count.insert(c, new_count);
    }

    let password_valid = match char_count.get(&target_char) {
        Some(count) => lo <= *count && hi >= *count,
        _ => lo == 0 && hi == 0
    };

    return password_valid;
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let num_valid = parse(input)?
    .map(|line: String| {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        if tokens.len() != 3 {
            return false;
        }

        let (first, second, password) = (tokens[0], tokens[1], tokens[2]);
        let (lo, hi) = parse_lo_hi(&first.to_string());
        let target_char = parse_target_char(&second.to_string());

        let lo_pos_has_char = char_at_ith_equals(&password.to_string(), (lo - 1) as usize, &target_char);
        let hi_pos_has_char = char_at_ith_equals(&password.to_string(), (hi - 1) as usize, &target_char);
        
        (lo_pos_has_char && !hi_pos_has_char) || (!lo_pos_has_char && hi_pos_has_char)

    })
    .filter(|&item| item )
    .count();

    println!("{}", num_valid);
    Ok(())
}

fn char_at_ith_equals(input: &String, idx: usize, target_char: &char) -> bool {
    if input.len() <= idx {
        return false;
    }
    input.chars().nth(idx).unwrap() == *target_char
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/* Alternative Solution

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{min_count}-{max_count} {char_counted}: {example}")]
struct PasswordPolicy {
    min_count: u32,
    max_count: u32,
    char_counted: char,
    example: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let ch_count = self
            .example
            .chars()
            .filter(|ch| *ch == self.char_counted)
            .count();
        ch_count >= self.min_count as usize && ch_count <= self.max_count as usize
    }

    fn is_valid_part2(&self) -> bool {
        let example = self.example.as_bytes();
        (example[self.min_count as usize - 1] == self.char_counted as u8)
            ^ (example[self.max_count as usize - 1] == self.char_counted as u8)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let n_valid = parse::<PasswordPolicy>(input)?
        .filter(|example| example.is_valid())
        .count();
    println!("{} valid passwords", n_valid);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let n_valid = parse::<PasswordPolicy>(input)?
        .filter(|example| example.is_valid_part2())
        .count();
    println!("{} valid passwords (part 2)", n_valid);
    Ok(())
}
*/