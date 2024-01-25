use std::{
    collections::HashSet,
    fs::{read, File},
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct PartNumber {
    number: u32,
    start: u32,
    end: u32,
}

/// This function solves the Part 2 problem. It works by reading in PartNumber data for 3 lines, and
/// '*' symbol data for two lines. On each iteration, the symbols for the previous line are anaylzed (because
/// you need the PartNumber data for the line you're currently on).
pub fn calc_sum_of_gear_ratios(path: &str) -> u32 {
    let mut sum = 0;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut prev_prev_parts: Vec<PartNumber> = Vec::new();
    let mut prev_parts: Vec<PartNumber> = Vec::new();
    let mut prev_asterisks: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let curr_parts = get_part_numbers(&line);
        for ast_pos in &prev_asterisks {
            let mut found_parts: Vec<u32> = Vec::new();
            let start = if *ast_pos == 0 { 0 } else { ast_pos - 1 };
            let end = if *ast_pos == (line.len() - 1) as u32 {
                line.len() as u32 - 1
            } else {
                ast_pos + 1
            };
            // check prev_prev parts
            for part in &prev_prev_parts {
                if part.start <= end && part.end >= start {
                    found_parts.push(part.number);
                }
            }
            // check prev_parts
            for part in &prev_parts {
                if part.start <= end && part.end >= start {
                    found_parts.push(part.number);
                }
            }
            // check curr_parts
            for part in &curr_parts {
                if part.start <= end && part.end >= start {
                    found_parts.push(part.number);
                }
            }

            if found_parts.len() == 2 {
                sum += found_parts[0] * found_parts[1];
            }
        }

        // update the collections before next iter
        prev_asterisks.clear();
        for (i, c) in line.chars().enumerate() {
            if c == '*' {
                prev_asterisks.push(i as u32);
            }
        }
        prev_prev_parts = prev_parts;
        prev_parts = curr_parts;
    }

    sum
}

/// This function solves the Part 1 problem
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
