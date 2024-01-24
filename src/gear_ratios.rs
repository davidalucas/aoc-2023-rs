#[derive(Debug, PartialEq)]
struct PartNumber {
    number: String,
    start: u32,
}

fn get_part_numbers(data: String) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();
    let mut number = String::new();
    let mut start: usize = 0;

    for (i, c) in data.chars().enumerate() {
        if c.is_digit(10) {
            if number.is_empty() {
                start = i;
            }
            number.push(c);
        } else if !number.is_empty() {
            part_numbers.push(PartNumber {
                number,
                start: start as u32,
            });
            number = String::new();
        }
    }

    if !number.is_empty() {
        part_numbers.push(PartNumber {
            number,
            start: start as u32,
        })
    }

    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_part_numbers_works_as_expected() {
        let actual = get_part_numbers(String::from("467..114.."));
        assert_eq!(
            actual,
            vec![
                PartNumber {
                    number: String::from("467"),
                    start: 0
                },
                PartNumber {
                    number: String::from("114"),
                    start: 5
                }
            ]
        )
    }
}
