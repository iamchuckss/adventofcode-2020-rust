use aoc2020::{parse};
use structopt::lazy_static::lazy_static;

use std::{path::Path};
use thiserror::Error;

lazy_static! {
    static ref ANGLE_TO_QUADRANT: Vec<Direction> = {
        use Direction::*;
        let mut v: Vec<Direction> = Vec::new();
        v.push(North);
        v.push(East);
        v.push(South);
        v.push(West);
        v
    };
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::FromStr, parse_display::Display)]
enum Instruction {
    #[display("N{0}")]
    North(i32),
    #[display("S{0}")]
    South(i32),
    #[display("E{0}")]
    East(i32),
    #[display("W{0}")]
    West(i32),
    #[display("L{0}")]
    Left(i32),
    #[display("R{0}")]
    Right(i32),
    #[display("F{0}")]
    Forward(i32)
}

#[derive (Clone, Copy, Default, Debug)]
struct Location {
    north: u32,
    south: u32,
    east: u32,
    west: u32,
}

#[derive (Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive (Clone)]
struct Ship {
    origin: Location,
    current_location: Location,
    angle: u32
}

impl Ship {
    pub fn new(origin: Location, angle: u32) -> Ship {
        Ship {
            origin,
            angle,
            current_location: origin.clone(),
        }
    }

    pub fn handle_instruction(&mut self, instruction: Instruction) -> Result<(), Error> {
        use Instruction::*;
        match instruction {
            North(val) => {
                self.forward_by_unit(Direction::North, val as u32);
            },
            East(val) => {
                self.forward_by_unit(Direction::East, val as u32);
            },
            South(val) => {
                self.forward_by_unit(Direction::South, val as u32);
            },
            West(val) => {
                self.forward_by_unit(Direction::West, val as u32);
            },
            Forward(val) => {
                self.forward_by_unit(self.get_direction(), val as u32);
            },
            Left(val) => {
                self.turn(-val);
            },
            Right(val) => {
                self.turn(val);
            }
        }
        Ok(())
    }

    pub fn get_direction(&self) -> Direction {
        let index: usize = (self.angle / 90) as usize;
        ANGLE_TO_QUADRANT[index]
    }

    fn turn(&mut self, degrees: i32) {
        let mut new_angle = (self.angle as i32) + degrees;
        if new_angle < 0 {
            new_angle += 360;
        }
        self.angle = (new_angle as u32) % 360;
    }

    fn forward_by_unit(&mut self, direction: Direction, distance: u32) {
        use Direction::*;
        match direction {
            North => {
                if distance > self.current_location.south {
                    self.current_location.north += distance - self.current_location.south;
                    self.current_location.south = 0;
                } else {
                    self.current_location.south -= distance;
                }
            },
            East => {
                if distance > self.current_location.west {
                    self.current_location.east += distance - self.current_location.west;
                    self.current_location.west = 0;
                } else {
                    self.current_location.west -= distance;
                }
            },
            South => {
                if distance > self.current_location.north {
                    self.current_location.south += distance - self.current_location.north;
                    self.current_location.north = 0;
                } else {
                    self.current_location.north -= distance;
                }
            },
            West => {
                if distance > self.current_location.east {
                    self.current_location.west += distance - self.current_location.east;
                    self.current_location.east = 0;
                } else {
                    self.current_location.east -= distance;
                }
            }
        }
    }
}

fn manhattan_distance(p1: Location, p2: Location) -> u32 {
    (p2.north + p2.south) + (p2.east + p2.west)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut ship = Ship::new(Location::default(), 90);
    for instruction in parse::<Instruction>(input)? {
        ship.handle_instruction(instruction)?;
    }
    
    println!("{}", manhattan_distance(ship.origin, ship.current_location));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut ship = Ship::new(Location::default(), 90);
    for instruction in parse::<Instruction>(input)? {
        ship.handle_instruction(instruction)?;
    }
    
    println!("{}", manhattan_distance(ship.origin, ship.current_location));
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
