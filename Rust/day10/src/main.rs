fn encode(input: String) -> String {
    let mut result = String::new();
    let mut prev = None;
    let mut count = 0;
    for c in input.chars() {
        if prev.is_none() || prev == Some(c) {
            count += 1;
        } else {
            result += &format!("{}{}", count, prev.unwrap());
            count = 1;
        }
        prev = Some(c)
    }
    result += &format!("{}{}", count, prev.unwrap());
    result
}

fn main() {
    let input = "1321131112".to_string();
    let mut part_a = input.clone();
    for _ in 0..50 {
        part_a = encode(part_a);
    }
    println!("part_a.len(): {}", part_a.len());
}
