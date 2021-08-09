/// Dia7
use std::collections::HashMap;

#[derive(Debug)]
struct Wire {
    name: Option<[char; 2]>,
    signal: Option<u16>,
}

impl Wire {
    fn new(name: Option<[char; 2]>, signal: Option<u16>) -> Self {
        Self { name, signal }
    }
}

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

// impl Operation {
//     fn from_str(input: &str) -> Self {
//         match input {
//             "AND"    => Self::AND,
//             "OR"     => Self::OR,
//             "NOT"    => Self::NOT,
//             "LSHIFT" => Self::LSHIFT,
//             "RSHIFT" => Self::RSHIFT,
//             _        => panic!("not a valid operation")
//         }
//     }
// }

// struct Gate {
//     operation: Operation,
// }

// fn parse_line(line: &str, hash: &HashMap<String, String>) {
//     let raw_words: Vec<_> = line.split(" -> ").collect();
// }

fn main() {
    let input = include_str!("../input_small.txt");
    let mut gates_map = HashMap::new();
    for line in input.lines() {
        let raw_words: Vec<_> = line.split(" -> ").collect();
        gates_map.insert(raw_words[1], raw_words[0]);
    }
    println!("gates_map: {:?}", gates_map);
}
