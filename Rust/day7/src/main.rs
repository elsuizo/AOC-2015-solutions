/// Day7
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

type Wire = &'static str;

#[derive(Debug, Clone, Copy)]
enum Operation {
    And(Wire, Wire),
    Or(Wire, Wire),
    Not(Wire),
    Lshift(Wire, u16),
    Rshift(Wire, u16),
    Set(u16),
    Single(Wire),
}

fn load_circuit(input: &'static str) -> Result<HashMap<Wire, Operation>, Box<dyn Error>> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let in_out: Vec<_> = line.split(" -> ").collect();
        let instruction = in_out[0];
        // single element input
        if !instruction.contains(" ") {
            // is numeric???
            if instruction.chars().all(char::is_numeric) {
                result.insert(in_out[1], Operation::Set(u16::from_str(instruction)?));
            } else {
                result.insert(in_out[1], Operation::Single(instruction));
            }
        } else {
            // this are all the others cases
            let operations: Vec<_> = instruction.split_whitespace().collect();
            if operations[1] == "AND" {
                result.insert(in_out[1], Operation::And(operations[0], operations[2]));
            } else if operations[1] == "OR" {
                result.insert(in_out[1], Operation::Or(operations[0], operations[2]));
            } else if operations[0] == "NOT" {
                result.insert(in_out[1], Operation::Not(operations[1]));
            } else if operations[1] == "LSHIFT" {
                let lhs = u16::from_str(operations[2])?;
                result.insert(in_out[1], Operation::Lshift(operations[0], lhs));
            } else if operations[1] == "RSHIFT" {
                let lhs = u16::from_str(operations[2])?;
                result.insert(in_out[1], Operation::Rshift(operations[0], lhs));
            }
        }
    }
    Ok(result)
}

fn simulate_circuit(wire: Wire, circuit: &mut HashMap<Wire, Operation>) -> u16 {
    // the wire is in the circuit
    if circuit.contains_key(wire) {
        let value = circuit[wire];
        let result = match value {
            Operation::Set(num) => num,
            Operation::Not(w) => !simulate_circuit(w, circuit),
            Operation::And(l, r) => simulate_circuit(l, circuit) & simulate_circuit(r, circuit),
            Operation::Or(l, r) => simulate_circuit(l, circuit) | simulate_circuit(r, circuit),
            Operation::Lshift(r, num) => simulate_circuit(r, circuit) << num,
            Operation::Rshift(r, num) => simulate_circuit(r, circuit) >> num,
            Operation::Single(w) => simulate_circuit(w, circuit),
        };
        circuit.insert(wire, Operation::Set(result));
        result
    } else {
        // this is the case when the input is numeric and is not in the map
        let result = u16::from_str(wire).unwrap();
        circuit.insert(wire, Operation::Set(result));
        result
    }
}

fn part1(input: &'static str, wire: Wire) -> Result<u16, Box<dyn Error>> {
    let mut circuit = load_circuit(input)?;
    Ok(simulate_circuit(wire, &mut circuit))
}

fn part2(input: &'static str, wire: Wire, override_value: u16) -> Result<u16, Box<dyn Error>> {
    let mut circuit = load_circuit(input)?;
    circuit.insert("b", Operation::Set(override_value));
    Ok(simulate_circuit(wire, &mut circuit))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let result1 = part1(input, "a")?;
    let result2 = part2(input, "a", result1)?;
    println!("result1: {:?}", result1);
    println!("result2: {:?}", result2);

    Ok(())
}
