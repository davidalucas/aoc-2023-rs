use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub mod map_range;
use map_range::*;

pub struct Almanac {
    pub seeds: Vec<i64>,
    pub seed_ranges: Vec<(i64, i64)>,
    pub maps: Vec<Vec<MapRange>>,
}

impl Almanac {
    pub fn from_file(path: &str) -> Almanac {
        let file = File::open(path).unwrap();
        let mut lines = BufReader::new(file).lines();

        let seed_line = lines.next().unwrap().unwrap();
        let seeds = parse_seeds(&seed_line);
        let seed_ranges = parse_seed_ranges(&seed_line);
        lines.next(); //progress one line

        let mut maps: Vec<Vec<MapRange>> = Vec::new();
        let mut range_stack: VecDeque<String> = VecDeque::new();
        for line in lines {
            let line = line.unwrap();
            if line == "" {
                range_stack.pop_front(); // we don't need the map title
                maps.push(make_map_ranges(&range_stack));
                range_stack.clear();
                continue;
            }
            range_stack.push_back(line);
        }

        //empty the queue
        range_stack.pop_front();
        maps.push(make_map_ranges(&range_stack));
        range_stack.clear();

        Almanac {
            seeds,
            seed_ranges,
            maps,
        }
    }

    /// Solves the Day 5 Part 1 problem
    pub fn get_lowest_location(&self) -> i64 {
        let mut lowest = i64::MAX;

        for seed in &self.seeds {
            let mut source = *seed;
            for map in &self.maps {
                let found = map.binary_search_by(|m| m.cmp(source));
                match found {
                    Err(_) => continue,
                    Ok(i) => {
                        let found_map = map.get(i).unwrap();
                        source = found_map.destination + (source - found_map.source);
                    }
                }
            }
            if source < lowest {
                lowest = source;
            }
        }

        lowest
    }

    /// Solves Day 5 Part 2 problem
    pub fn get_lowest_location_ranged(&self) -> i64 {
        let mut lowest = i64::MAX;

        for (seed, range) in &self.seed_ranges {
            let mut source = *seed;

            while source < seed + range {
                let mut curr_source = source;
                let mut curr_range = range - (curr_source - seed);
                for map in &self.maps {
                    let found = map.binary_search_by(|m| m.cmp(curr_source));
                    match found {
                        Ok(i) => {
                            let found_map = map.get(i).unwrap();
                            let offset = curr_source - found_map.source;
                            let found_map_adj_range = found_map.range - offset;

                            curr_source = found_map.destination + offset;
                            if found_map_adj_range < curr_range {
                                curr_range = found_map_adj_range
                            }
                        }
                        Err(i) => {
                            if i == map.len() {
                                continue;
                            }

                            let found_map = map.get(i).unwrap();
                            let found_map_adj_range = found_map.source - curr_source;
                            if found_map_adj_range < curr_range {
                                curr_range = found_map_adj_range;
                            }
                        }
                    }
                }
                if curr_source < lowest {
                    lowest = curr_source;
                }
                source += curr_range;
            }
        }

        lowest
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let seeds_str = &line["seeds: ".len()..];
    seeds_str
        .split(" ")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_seed_ranges(line: &str) -> Vec<(i64, i64)> {
    let seeds_strings = &line["seeds: ".len()..].split(" ").collect::<Vec<&str>>();
    let mut seed_ranges = Vec::new();
    let mut count = 0;

    while count < seeds_strings.len() {
        let start = seeds_strings.get(count).unwrap().parse().unwrap();
        let range = seeds_strings.get(count + 1).unwrap().parse().unwrap();
        seed_ranges.push((start, range));
        count += 2;
    }

    seed_ranges
}

/// Takes in a queue of MapRange strings and returns a Vec<MapRange>, sorted by their
/// 'source' fields.
fn make_map_ranges(range_stack: &VecDeque<String>) -> Vec<MapRange> {
    let mut ranges = range_stack
        .iter()
        .map(|s| MapRange::from(&s))
        .collect::<Vec<MapRange>>();
    ranges.sort_by(|a, b| a.source.cmp(&b.source));
    ranges
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn parse_seeds_works_as_expected() {
        let line = "seeds: 79 14 55 13";
        let expected = Vec::from([79, 14, 55, 13]);
        let actual = parse_seeds(line);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_seed_ranges_works_as_expected() {
        let line = "seeds: 79 14 55 13";
        let expected = Vec::from([(79, 14), (55, 13)]);
        let actual = parse_seed_ranges(line);

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_map_ranges_works_as_expected() {
        let mut range_stack = VecDeque::from([String::from("50 98 2"), String::from("52 50 48")]);
        let actual = make_map_ranges(&mut range_stack);
        let expected = Vec::from([
            MapRange {
                source: 50,
                destination: 52,
                range: 48,
            },
            MapRange {
                source: 98,
                destination: 50,
                range: 2,
            },
        ]);

        assert_eq!(actual, expected);
        assert_eq!(range_stack.len(), 2);
    }
}
