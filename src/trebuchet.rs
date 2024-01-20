use std::{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_returns_expected_integer() {
        let result = parse_digits("two1nine");
        assert_eq!(result, 11);
    }
}
