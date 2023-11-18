#[derive(Debug)]
struct Password {
    actual: String,
    is_valid: bool,
}

#[derive(Debug)]
struct PasswordIter {
    state: Password,
}

impl Iterator for PasswordIter {
    type Item = Password;
    fn next(&mut self) -> Option<Self::Item> {}
}

impl Password {
    fn get_actual(&self) -> &str {
        &self.actual
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

// TODO(elsuizo: 2023-03-20): esto se tiene que poder hacer de otra manera mas elegante...
/// check if the three
fn contains_bad_set(input: &str, bad_set: &[char]) -> bool {
    for c in input.chars() {
        for bad_c in bad_set {
            if c == *bad_c {
                return true;
            }
        }
    }
    false
}

/// check if are three consecutives
fn contains_3_consecutive(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|w| w[0] == w[1] - 1 && w[0] == w[2] - 2)
}

/// check if two letters are equal in the input
fn contains_consecutive_pairs(input: &str) -> bool {
    input.as_bytes().windows(2).filter(|c| c[0] == c[1]).count() >= 2
}

fn main() {
    let input = "zbxkghblabddciabzz";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement1() {
        let input = "hijklmmn";
        let result = contains_3_consecutive(input);
        assert!(result)
    }

    #[test]
    fn requirement2() {
        let input = "abbceffg";
        let input2 = "asdfasdi";
        let result = contains_bad_set(input, &['i', 'o', 'l']);
        let result2 = contains_bad_set(input2, &['i', 'o', 'l']);
        assert!(!result);
        assert!(result2)
    }

    #[test]
    fn requirement3() {
        let input = "abbceffg";
        let result = contains_consecutive_pairs(input);
        assert!(result)
    }
}
