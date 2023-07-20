use std::collections::{HashMap, HashSet};

pub fn run() {
    let file = match std::fs::read_to_string("input/day-3.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Could not open the file. Error: {}", error),
    };

    let scorer = Scorer::new();

    let total: u32 = file
        .lines()
        .map(|line| {
            let rucksack = split_to_rucksack(line);
            let duplicate = rucksack.duplicate().expect("should have duplicate");

            scorer
                .score_letter(duplicate)
                .expect("should be able to score letter") as u32
        })
        .sum();

    println!("Total score: {total}");
}

#[derive(Debug, PartialEq, Eq)]
struct Rucksack {
    first: String,
    second: String,
}

impl Rucksack {
    fn duplicate(&self) -> Option<char> {
        let first: HashSet<_> = self.first.chars().collect();
        let second: HashSet<_> = self.second.chars().collect();

        first.intersection(&second).next().copied()
    }
}

fn split_to_rucksack(text: &str) -> Rucksack {
    let len = text.len();

    let first = &text[..(len / 2)];
    let second = &text[(len / 2)..];

    Rucksack {
        first: first.to_string(),
        second: second.to_string(),
    }
}

struct Scorer {
    lookup: HashMap<char, u8>,
}

impl Scorer {
    fn new() -> Self {
        let mut alpha = "abcdefghijklmnopqrstuvwxyz".to_string();
        alpha += &alpha.to_uppercase();

        let lookup: HashMap<char, u8> = alpha
            .chars()
            .enumerate()
            .map(|(idx, letter)| (letter, idx as u8 + 1))
            .collect();

        Self { lookup }
    }

    fn score_letter(&self, letter: char) -> Option<u8> {
        self.lookup.get(&letter).copied()
    }
}

fn score_letter_linear_scan(letter: char) -> Option<u8> {
    let mut alpha = "abcdefghijklmnopqrstuvwxyz".to_string();
    alpha += &alpha.to_uppercase();
    alpha.find(letter).map(|v| v as u8 + 1)
}

fn score_letter_fast(letter: char) -> u8 {
    let byte = letter as u8;
    if byte >= 97 {
        byte - 96
    } else {
        (byte - 65) + 27
    }
}

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_rucksak() {
        let actual = split_to_rucksack("abcdef");
        let expected = Rucksack {
            first: "abc".to_string(),
            second: "def".to_string(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn rucksack_duplicates() {
        let rucksack = Rucksack {
            first: "abc".to_string(),
            second: "cde".to_string(),
        };

        let actual = rucksack.duplicate();
        let expected = Some('c');

        assert_eq!(actual, expected);
    }

    #[test]
    fn rucksack_no_duplicates() {
        let rucksack = Rucksack {
            first: "abc".to_string(),
            second: "def".to_string(),
        };

        let actual = rucksack.duplicate();
        let expected = None;

        assert_eq!(actual, expected);
    }

    #[test]
    fn score_lowercase_letter() {
        let scorer = Scorer::new();

        let letters = vec!['a', 'b', 'c', 'y', 'z'];
        let scores = vec![1, 2, 3, 25, 26];

        let actual: Vec<_> = letters
            .iter()
            .map(|letter| scorer.score_letter(*letter).unwrap())
            .collect();

        assert_eq!(actual, scores);
    }

    #[test]
    fn score_fast_letter() {
        let scorer = Scorer::new();

        let letters = vec!['a', 'b', 'c', 'y', 'z'];
        let scores = vec![1, 2, 3, 25, 26];

        let actual: Vec<_> = letters
            .iter()
            .map(|letter| score_letter_fast(*letter))
            .collect();

        assert_eq!(actual, scores);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_find_linear(b: &mut Bencher) {
        b.iter(|| score_letter_linear_scan('A'));
    }

    #[bench]
    fn bench_find_linear_slow(b: &mut Bencher) {
        b.iter(|| score_letter_linear_scan('Z'));
    }

    #[bench]
    fn bench_find_lookup(b: &mut Bencher) {
        let scorer = Scorer::new();
        b.iter(|| scorer.score_letter('A'));
    }

    #[bench]
    fn bench_find_fast(b: &mut Bencher) {
        b.iter(|| score_letter_fast('A'));
    }
}
