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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_returns_expected_integer() {
        let result = parse_digits("two1nine");
        assert_eq!(result, 11);
    }
}
