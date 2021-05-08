use aoc2020::{parse};

use std::{collections::{HashSet}, path::Path};
use thiserror::Error;

enum Instruction {
    Nop,
    Jmp(i32),
    Acc(i32)
}

struct InstructionSet {
    instructions: Vec<Instruction>,
    visited_instructions: HashSet<usize>,
    ptr: usize,
    accumulator: i32
}

impl InstructionSet {
    pub fn new(instructions: Vec<Instruction>) -> InstructionSet {
        InstructionSet {
            instructions,
            visited_instructions: HashSet::new(),
            ptr: 0,
            accumulator: 0
        }
    }

    pub fn get_accumulator(&self) -> i32 {
        self.accumulator
    }

    pub fn execute_next(&mut self) -> bool {
        if self.ptr >= self.instructions.len() { 
            return false; 
        }
        if self.visited_instructions.contains(&self.ptr) { return false; }

        self.visited_instructions.insert(self.ptr);
        let next_instr = &self.instructions[self.ptr];
        match next_instr {
            Instruction::Nop => {
                self.ptr += 1;
            },
            Instruction::Acc(val) => {
                self.accumulator += *val;
                self.ptr += 1;
            },
            Instruction::Jmp(val) => {
                if val.is_negative() {
                    self.ptr -= val.wrapping_abs() as usize;
                } else {
                    self.ptr += *val as usize;
                }
            },
        }
        return true;
    }
}


pub fn part1(input: &Path) -> Result<(), Error> {
    let instructions = parse(input)?
        .map(|line: String| {
            let mut tokens = line.split_whitespace();
            let instruction_str = tokens.next().unwrap();
            let instruction_val = tokens.next().unwrap().parse::<i32>().unwrap();
            
            let instruction: Instruction = match instruction_str {
                "nop" => { Instruction::Nop },
                "acc" => { Instruction::Acc(instruction_val) },
                "jmp" => { Instruction::Jmp(instruction_val) },
                _ => { Instruction::Nop },
            };

            instruction
        }).collect::<Vec<Instruction>>();
    
    let mut instruction_set: InstructionSet = InstructionSet::new(instructions);
    
    loop {
        if !instruction_set.execute_next() {
            break;
        }
    }

    println!("{}", instruction_set.get_accumulator());

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
   
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}
