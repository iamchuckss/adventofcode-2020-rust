use aoc2020::parse;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut input = parse::<String>(input)?;
    let earliest_timestamp = input.next().expect("Wrong format").parse::<u32>().unwrap();
    let bus_timestamps = input.next().expect("Wrong format");
    let bus_timestamps = bus_timestamps.split(',')
        .filter(|timestamp| *timestamp != "x")
        .map(|timestamp| timestamp.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let bus_waiting_times = bus_timestamps.iter()
        .map(|timestamp| {
            let mut counter = 1;
            let mut bus_earliest_timestamp = *timestamp;
            while bus_earliest_timestamp < earliest_timestamp {
                bus_earliest_timestamp = timestamp * counter;
                counter += 1;
            }
            bus_earliest_timestamp - earliest_timestamp
        });
    
    let (min_index, min_waiting_time) = bus_waiting_times.enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    
    let answer = bus_timestamps[min_index] * min_waiting_time;
    println!("{:?}", answer);

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
