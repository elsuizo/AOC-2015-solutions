fn parse_list(list_input: &str) -> (usize, usize) {}

fn main() {
    let test = "aaa\"aaa";
    let test2 = "\x27";
    let mut counter = 0;
    for c in test.chars() {
        counter += 1;
    }
    println!("counter: {:?}", counter);
    println!("size of test: {}", test.len());
}
