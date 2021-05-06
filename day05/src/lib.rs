use aoc2020::parse;

use std::{path::Path};
use thiserror::Error;

#[derive (Default)]
struct Seat {
    id: i32,
    row: i32,
    col: i32,
    binary: String
}

impl Seat {
    pub fn new(binary: &str) -> Seat {
        let row: i32 = Seat::resolve_binary_partition(&binary[..7], 0, 128);
        let col: i32 = Seat::resolve_binary_partition(&binary[7..], 0, 8);
        let id: i32 = Seat::get_id(row, col);

        Seat { id, row, col, binary: binary.to_string() }
    }

    fn resolve_binary_partition(binary: &str, lo: i32, hi: i32) -> i32 {
        let input_len =  binary.to_string().len();

        let mid: i32 = lo + (hi - lo) / 2;
        match binary.chars().nth(0) {
            Some('F') | Some('L') => { 
                if input_len == 1 { return lo; }
                return Seat::resolve_binary_partition(&binary[1..], lo, mid); 
            },
            Some('B') | Some('R') => { 
                if input_len == 1 { return hi - 1; }
                return Seat::resolve_binary_partition(&binary[1..], mid, hi); 
            },
            _ => { return 0; }
        }
    }

    fn get_id(row: i32, col: i32) -> i32 {
        row * 8 + col
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let max_seat: Option<Seat> = parse(input)?
        .map(|line: String| { Seat::new(line.as_str()) })
        .max_by(|s1, s2| s1.id.cmp(&s2.id));

    if let Some(max_seat) = max_seat {
        println!("{}", max_seat.id);
    }
    
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let sorted_ids = parse(input)?
        .map(|line: String| { Seat::new(line.as_str()).id })
        .sorted();

    let mut prev = 0;
    for id in sorted_ids {
        if prev != 0 && id != prev + 1 {
            println!("{}", id - 1);
            break;
        }
        prev = id;
    }
     
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