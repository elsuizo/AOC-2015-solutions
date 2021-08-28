use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let mut cities = HashSet::new();
    let mut pairs = HashMap::new();

    for line in input.lines() {
        // parsing the data
        let raw_data: Vec<_> = line.split_whitespace().collect();
        cities.insert(raw_data[0]);
        cities.insert(raw_data[2]);
        // we need the two combinations of cities
        pairs.insert((raw_data[0], raw_data[2]), usize::from_str(raw_data[4])?);
        pairs.insert((raw_data[2], raw_data[0]), usize::from_str(raw_data[4])?);
    }
    // distances calculations

    let mut distances = Vec::new();
    // we need all the permutations for the cities for that we could use the very nice itertool crate
    for path in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        let mut prev: Option<&str> = None;
        for current in path {
            if let Some(prev) = prev {
                distance += pairs[&(prev, *current)];
            }
            prev = Some(current);
        }
        distances.push(distance);
    }
    let min_value = distances.iter().min();
    let max_value = distances.iter().max();

    println!("min_value: {:?}", min_value);
    println!("max_value: {:?}", max_value);

    Ok(())
}
