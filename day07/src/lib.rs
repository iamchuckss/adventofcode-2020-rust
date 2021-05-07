use aoc2020::{input::parse_newline_sep, parse};
use itertools::Itertools;

use std::{collections::{HashMap, HashSet}, num, path::Path};
use thiserror::Error;
use regex::Regex;

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{bag} bags contain {children_string}.")]
struct BagRule {
    bag: String,
    children_string: String
}

struct BagGraph {
    graph: HashMap<String, HashMap<String, i32>>
}

impl BagGraph {
    pub fn has_n_bag(&self, bag: &str, target: &str, num_item: i32) -> bool {
        if !self.graph.contains_key(&bag.to_string()) { return false; }

        let current_bag = self.graph.get(&bag.to_string()).unwrap();
        
        match current_bag.get(&target.to_string()) { 
            Some(val) => { 
                return *val >= num_item; 
            },
            None => { 
                return current_bag.keys()
                    .any(|inner_bag: &String| BagGraph::has_n_bag(&self, inner_bag.as_str(), target, num_item)); 
            }
        }        
    }

    pub fn sum_bags(&self, bag:&str) -> i32 {
        if !self.graph.contains_key(&bag.to_string()) { return 0; }

        let current_bag = self.graph.get(&bag.to_string()).unwrap();
        
        return current_bag.iter()
            .map(|(inner_bag, num)| {
                *num + *num * BagGraph::sum_bags(&self, inner_bag.as_str())
            })
            .sum(); 
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let rules = parse::<BagRule>(input)?;

    let mut graph: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let re = Regex::new(r"(\d+) (.*) [bag|bags]").unwrap();
    
    for rule in rules {
        let inner_graph = graph.entry(rule.bag.clone()).or_insert(HashMap::new());
        let tokens = rule.children_string.split(",");
    
        for token in tokens {
            let captured = re.captures(token.trim())
                .map(|cap| (cap[2].to_string(), cap[1].parse::<i32>().unwrap()));
            
            match captured {
                Some((inner_bag, num_item)) => { inner_graph.insert(inner_bag, num_item ); },
                _ => {}
            }
        }
    }

    let bag_graph = BagGraph {graph: graph};

    let count = bag_graph.graph.keys().filter(|bag| {
        bag_graph.has_n_bag(bag.as_str(), "shiny gold", 1)
    }).count();
    
    dbg!(count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let rules = parse::<BagRule>(input)?;

    let mut graph: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let re = Regex::new(r"(\d+) (.*) [bag|bags]").unwrap();
    
    for rule in rules {
        let inner_graph = graph.entry(rule.bag.clone()).or_insert(HashMap::new());
        let tokens = rule.children_string.split(",");
    
        for token in tokens {
            let captured = re.captures(token.trim())
                .map(|cap| (cap[2].to_string(), cap[1].parse::<i32>().unwrap()));
            
            match captured {
                Some((inner_bag, num_item)) => { inner_graph.insert(inner_bag, num_item ); },
                _ => {}
            }
        }
    }

    let bag_graph = BagGraph {graph: graph};

    let count = bag_graph.sum_bags("shiny gold");
    
    dbg!(count);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid argument(s) `{0}`")]
    InvalidArgs(String),
}
