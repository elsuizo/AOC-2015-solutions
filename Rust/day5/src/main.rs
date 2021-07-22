use std::collections::HashMap;

fn contains_vowels(input: &str) -> bool {
    let mut vowels: HashMap<char, usize> = HashMap::new();
    vowels.insert('a', 0);
    vowels.insert('e', 0);
    vowels.insert('i', 0);
    vowels.insert('o', 0);
    vowels.insert('u', 0);
    for c in input.chars() {
        *vowels.entry(c).or_insert(1) += 1;
    }
    vowels[&'a'] + vowels[&'e'] + vowels[&'i'] + vowels[&'o'] + vowels[&'u'] >= 3
}

fn contains_twice(input: &str) -> bool {
    for b in input.as_bytes().windows(2) {
        if b[0] == b[1] {
            return true;
        }
    }
    false
}

fn contains_set(input: &str) -> bool {
    let set = ["ab", "cd", "pq", "xy"];
    for s in set {
        if input.contains(s) {
            return true;
        }
    }
    false
}

fn contains_between(input: &str) -> bool {
    for b in input.as_bytes().windows(3) {
        if b[0] == b[2] {
            return true;
        }
    }
    false
}

fn contains_pairs(input: &str) -> bool {
    let mut pairs: HashMap<&[u8], usize> = HashMap::new();
    for b in input.as_bytes().windows(2) {
        *pairs.entry(b).or_insert(1) += 1;
    }
    pairs.values().any(|&number| number > 2)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !contains_set(line) && contains_twice(line) && contains_vowels(line))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| contains_pairs(line) && contains_between(line))
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("the number of string that are nice is: {}", part1(&input));
    println!("the number of string that are nice is: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_vowels_test() {
        let input1 = "aeiouaeiouaeiou";
        let input2 = "777777777";

        assert!(contains_vowels(&input1));
        assert!(!contains_vowels(&input2));
    }

    #[test]
    fn contains_twice_test() {
        let input1 = "abcdde";
        let input2 = "absklop;";
        assert!(contains_twice(input1));
        assert!(!contains_twice(input2));
    }

    #[test]
    fn contains_set_test() {
        let input1 = "laksdjflpqaskdjf";
        let input2 = "lasdla";
        assert!(contains_set(&input1));
        assert!(!contains_set(&input2));
    }

    #[test]
    fn contains_between_test() {
        let input1 = "xyx";
        let input2 = "abcdde";
        assert!(contains_between(&input1));
        assert!(!contains_between(&input2))
    }

    #[test]
    fn contains_pairs_test() {
        let input1 = "aabcdefgaa";
        let input2 = "xyxy";
        assert!(contains_pairs(&input1));
        assert!(contains_pairs(&input2));
    }

    // TODO(elsuizo:2021-07-21): no se porque no anda si los test anda todo bien...
    #[test]
    fn part2_test() {
        let input1 = "qjhvhtzxzqqjkmpb";
        let input2 = "xxyxx";
        let input3 = "uurcxstgmygtbstg";
        let input4 = "ieodomkazucvgmuy";
        assert_eq!(part2(&input1), 1);
        assert_eq!(part2(&input2), 1);
        assert_eq!(part2(&input3), 0);
        assert_eq!(part2(&input4), 0);
    }
}
