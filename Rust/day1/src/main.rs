// TODO(elsuizo:2021-06-11): hacerlo de otra manera...
fn main() {
    let input = include_str!("../input.txt");

    let mut counter = 0;
    let mut idx = 0;
    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => {
                if counter != -1 {
                    counter += 1;
                } else {
                    idx = index;
                    break
                }
            },
            ')' => {
                if counter != -1 {
                    counter -= 1;
                } else {
                    idx = index;
                    break
                }
            },
            _  => {
                if counter != -1 {
                    counter += 0;
                } else {
                    idx = index;
                    break
                }
            }
        }
    }

    println!("counter: {}, index: {}", counter, idx);
}
