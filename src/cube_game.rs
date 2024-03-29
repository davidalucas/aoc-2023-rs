use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

/**
Performs the algorithm described in the Day 2 Part 1 problem description. Accepts a path to the file containing the
game data, as well as a HashMap containing the red/green/blue color limitations, and returns a sum of the game numbers
for all of the games which were possible according to the provided limitations.

# Examples

```
use std::collections::HashMap;
use aoc_2023_rs::cube_game::*;

let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
let result = sum_valid_games("./data/day_2/example.txt", color_limits).unwrap();

assert_eq!(result, 8);
```
*/
pub fn sum_valid_games(path: &str, color_limits: HashMap<&str, u16>) -> Result<usize> {
    let mut sum = 0;

    let file = File::open(path)?;
    for line in BufReader::new(file).lines().enumerate() {
        match line.1 {
            Err(err) => return Err(err),
            Ok(l) => {
                if game_is_valid(l, &color_limits) {
                    sum += line.0 + 1;
                }
            }
        }
    }

    Ok(sum)
}

/**
Determines if the provided game data (in raw string format) is valid or invalid according
to the provided cube color limits.
*/
fn game_is_valid(data: String, color_limits: &HashMap<&str, u16>) -> bool {
    let trimmed_data = data.split(": ").collect::<Vec<_>>(); // ["Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
    let reveals = trimmed_data[1].split("; ").collect::<Vec<_>>(); // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]

    for reveal in reveals {
        let cube_sums = reveal.split(", ").collect::<Vec<_>>(); // ["3 blue", "4 red"]
        for cube_data in cube_sums {
            let color_data = cube_data.split(' ').collect::<Vec<_>>(); // ["3", "blue"]
            let color_count = color_data[0].parse::<u16>().unwrap(); // 3
            if color_count > color_limits[color_data[1]] {
                return false;
            }
        }
    }

    true
}

/**
Performs the algorithm described in the Day 2 Part 2 problem description. Accepts a path to the file containing the
game data, and returns a sum of all of the 'powers' of all of the games.

# Examples

```
use aoc_2023_rs::cube_game::*;

let result = sum_game_powers("./data/day_2/example.txt").unwrap();
assert_eq!(result, 2286);
```
*/
pub fn sum_game_powers(path: &str) -> Result<usize> {
    let mut sum = 0;

    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        match line {
            Err(err) => return Err(err),
            Ok(l) => sum += calc_power(l),
        }
    }

    Ok(sum)
}

fn calc_power(data: String) -> usize {
    let mut min_colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let trimmed_data = data.split(": ").collect::<Vec<_>>();
    let reveals = trimmed_data[1].split("; ").collect::<Vec<_>>();

    for reveal in reveals {
        let cube_sums = reveal.split(", ").collect::<Vec<_>>();
        for cube_data in cube_sums {
            let color_data = cube_data.split(' ').collect::<Vec<_>>(); // ["3", "blue"]
            let color_count = color_data[0].parse().unwrap(); // 3
            if color_count > min_colors[color_data[1]] {
                min_colors.insert(color_data[1], color_count);
            }
        }
    }

    min_colors.values().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_is_valid_returns_true_when_expected() {
        let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let result = game_is_valid(
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            &color_limits,
        );

        assert_eq!(result, true);
    }

    #[test]
    fn game_is_valid_returns_false_when_expected() {
        let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let result = game_is_valid(
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            &color_limits,
        );

        assert_eq!(result, false);
    }

    #[test]
    fn calc_power_returns_expected_value() {
        let mut result = calc_power(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        ));
        assert_eq!(result, 48);

        result = calc_power(String::from(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        ));
        assert_eq!(result, 630);
    }
}
