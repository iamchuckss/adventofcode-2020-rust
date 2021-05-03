use aoc2020::parse;

use std::{path::Path};
use thiserror::Error;

struct Forest {
    rows: Vec<Vec<u8>>
}

const TREE_CHAR: char = '#';

impl Forest {
    pub fn count_trees(&self, col_incr: usize, row_incr: usize) -> Result<i32, Error> {
        let mut n_trees = 0;
        let mut loop_counter: usize = 0;
        for row_idx in (row_incr..self.rows.len()).step_by(row_incr) {
            loop_counter += 1;
            let curr_row = self.rows.get(row_idx).ok_or(Error::InvalidArgs(format!("{}", row_idx)))?;
            let col_idx = (loop_counter * col_incr) % curr_row.len();
            
            let curr_char = curr_row.get(col_idx).ok_or(Error::InvalidArgs(format!("{}", row_idx)))?;
            if *curr_char == TREE_CHAR as u8 {
                n_trees += 1;
            }
        }
        Ok(n_trees)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let rows: Vec<Vec<u8>> = parse(input)?
        .map(|row: String| { row.into_bytes() })
        .collect();

    let forest = Forest {rows};
    let n_trees = forest.count_trees(3, 1)?;

    println!("{}", n_trees);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let rows: Vec<Vec<u8>> = parse(input)?
        .map(|row: String| { row.into_bytes() })
        .collect();

    let forest = Forest {rows};
    let args = [
        (1, 1),
        (3, 1),
        (5, 1), 
        (7, 1),
        (1, 2)
    ];

    let n_trees: Vec<_> = args.iter().map(|(col_incr, row_incr)| {
        forest.count_trees(*col_incr as usize, *row_incr as usize)
    })
    .flatten()
    .collect();

    println!("{:?}", n_trees);    
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::FromStr, parse_display::Display)]
enum Tile {
    #[display("#")]
    Tree,
    #[display(".")]
    Clear,
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

struct XWrapMap(Map<Tile>);

impl std::ops::Index<Point> for XWrapMap {
    type Output = Tile;

    fn index(&self, mut point: Point) -> &Tile {
        point.x %= self.0.width().try_into().unwrap_or(i32::MAX);
        self.0.index(point)
    }
}

fn count_trees(map: &XWrapMap, slope: Point) -> u64 {
    let mut check = map.0.top_left();

    let mut n_trees = 0;
    while check.y >= 0 {
        if map[check] == Tile::Tree {
            n_trees += 1;
        }
        check += slope;
    }

    n_trees
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let map = XWrapMap(Map::try_from(input)?);
    let slope = Point::new(3, -1);
    let n_trees = count_trees(&map, slope);

    println!("trees encountered: {}", n_trees);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let map = XWrapMap(Map::try_from(input)?);

    let slopes = [
        Point::new(1, -1),
        Point::new(3, -1),
        Point::new(5, -1),
        Point::new(7, -1),
        Point::new(1, -2),
    ];

    let product: u64 = slopes
        .iter()
        .map(|&slope| count_trees(&map, slope))
        .product();

    println!("product of trees encountered: {}", product);

    Ok(())
}
*/