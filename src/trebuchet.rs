use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// Gets the first and last digit out of the provided string, returning them pushed together as a
/// single, two-digit number.
fn parse_digits(data: &str) -> u8 {
    let first_digit: u8 = data
        .matches(char::is_numeric)
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let second_digit: u8 = data
        .matches(char::is_numeric)
        .last()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    first_digit * 10 + second_digit
}

/// Performs the digit summation described in the Day 1 Part 1 problem.
pub fn sum_digits(path: &str) -> u64 {
    let mut sum: u64 = 0;

    if let Ok(file) = File::open(path) {
        for line in BufReader::new(file).lines() {
            match line {
                Ok(data) => sum += parse_digits(&data) as u64,
                Err(err) => panic!("{}", err),
            }
        }
    }

    sum
}

fn parse_digits_enhanced(data: &str) -> u8 {
    let num_map: HashMap<&str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let first_digit = find_first_digit(data, &num_map);
    let second_digit = find_last_digit(data, &num_map);

    first_digit * 10 + second_digit
}

fn find_first_digit(data: &str, num_map: &HashMap<&str, u8>) -> u8 {
    for (i, ch) in data.chars().enumerate() {
        if ch.is_ascii_digit() {
            return ch.to_digit(10).unwrap() as u8;
        }
        for (key, val) in num_map {
            if i + key.len() > data.len() {
                continue;
            }
            if &data[i..key.len()] == *key {
                return *val;
            }
        }
    }
    0
}

fn find_last_digit(data: &str, num_map: &HashMap<&str, u8>) -> u8 {
    for (i, ch) in data.chars().rev().enumerate() {
        if ch.is_ascii_digit() {
            return ch.to_digit(10).unwrap() as u8;
        }
        for (key, val) in num_map {
            let curr_pos = data.len() - i - 1;
            if key.len() > curr_pos {
                continue;
            }
            let start_idx = curr_pos - key.len() + 1;
            let sub_str = &data[start_idx..(curr_pos + 1)];
            if sub_str == *key {
                return *val;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_returns_expected_integer() {
        let result = parse_digits("two1nine");
        assert_eq!(result, 11);
    }

    #[test]
    fn find_first_digit_finds_first() {
        let num_map: HashMap<&str, u8> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let result = find_first_digit("two1nine", &num_map);
        assert_eq!(result, 2);
    }

    #[test]
    fn find_last_digit_finds_last() {
        let num_map: HashMap<&str, u8> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let result = find_last_digit("two1nine", &num_map);
        assert_eq!(result, 9);
    }

    #[test]
    fn parse_digits_enhanced_returns_expected_integer() {
        let result = parse_digits_enhanced("two1nine");
        assert_eq!(result, 29);
    }
}
