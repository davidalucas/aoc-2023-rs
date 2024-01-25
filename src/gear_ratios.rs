use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct PartNumber {
    number: u32,
    start: u32,
    end: u32,
}

pub fn calc_sum_of_part_numbers(path: &str) -> u32 {
    let mut sum = 0;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut prev_prev_symbols: HashSet<u32> = HashSet::new();
    let mut prev_symbols: HashSet<u32> = HashSet::new();
    let mut prev_parts: Vec<PartNumber> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let curr_parts = get_part_numbers(&line);
        let curr_symbols = get_symbols(&line);
        for part in prev_parts {
            let start = if part.start == 0 { 0 } else { part.start - 1 };
            let end = if part.end == line.len() as u32 - 1 {
                line.len() as u32 - 1
            } else {
                part.end + 1
            };
            // check prev_prev_symbols to see if adjacent to number
            if (start..=end).any(|i| prev_prev_symbols.contains(&i)) {
                sum += part.number;
                continue;
            }
            // check prev_part LH and RH sides for validation (remove from Vec once validated)
            if prev_symbols.contains(&start) || prev_symbols.contains(&end) {
                sum += part.number;
                continue;
            }
            // check curr_symbols to see if adjacent to number
            if (start..=end).any(|i| curr_symbols.contains(&i)) {
                sum += part.number;
                continue;
            }
        }
        // shift everything back one
        prev_prev_symbols = prev_symbols;
        prev_symbols = curr_symbols;
        prev_parts = curr_parts;
    }

    // handle last line of parts
    for part in prev_parts {
        // check prev_prev_symbols to see if adjacent to number
        if (part.start - 1..=part.end + 1).any(|i| prev_prev_symbols.contains(&i)) {
            sum += part.number;
            continue;
        }
        // check prev_part LH and RH sides for validation (remove from Vec once validated)
        if prev_symbols.contains(&(part.start - 1)) || prev_symbols.contains(&(part.end + 1)) {
            sum += part.number;
            continue;
        }
    }

    sum
}

fn get_part_numbers(data: &str) -> Vec<PartNumber> {
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
                number: number.parse().unwrap(),
                start: start as u32,
                end: (start + number.len() - 1) as u32,
            });
            number = String::new();
        }
    }

    if !number.is_empty() {
        part_numbers.push(PartNumber {
            number: number.parse().unwrap(),
            start: start as u32,
            end: (start + number.len() - 1) as u32,
        })
    }

    part_numbers
}

fn get_symbols(data: &str) -> HashSet<u32> {
    let mut sym_positions = HashSet::new();

    for (i, c) in data.chars().enumerate() {
        if !c.is_digit(10) && c != '.' {
            sym_positions.insert(i as u32);
        }
    }

    sym_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_part_numbers_works_as_expected() {
        let actual = get_part_numbers("467..114..");
        assert_eq!(
            actual,
            vec![
                PartNumber {
                    number: 467,
                    start: 0,
                    end: 2
                },
                PartNumber {
                    number: 114,
                    start: 5,
                    end: 7
                }
            ]
        );

        let actual = get_part_numbers(".....+.58.");
        assert_eq!(
            actual,
            vec![PartNumber {
                number: 58,
                start: 7,
                end: 8
            }]
        );
    }

    #[test]
    fn get_symbols_works_as_expected() {
        let actual = get_symbols("467..114..");
        assert_eq!(actual, HashSet::new());

        let actual = get_symbols("...$.*....");
        assert_eq!(actual, HashSet::from([3, 5]));

        let actual = get_symbols(".....+.58.");
        assert_eq!(actual, HashSet::from([5]));
    }
}
