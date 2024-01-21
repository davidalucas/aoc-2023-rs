use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

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

fn game_is_valid(data: String, color_limits: &HashMap<&str, u16>) -> bool {
    let trimmed_data = data.split(": ").collect::<Vec<_>>(); // ["Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
    let reveals = trimmed_data.get(1).unwrap().split("; ").collect::<Vec<_>>(); // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]

    for reveal in reveals {
        let cube_sums = reveal.split(", ").collect::<Vec<_>>(); // ["3 blue", "4 red"]
        for cube_data in cube_sums {
            let color_data = cube_data.split(' ').collect::<Vec<_>>(); // ["3", "blue"]
            let color_count = color_data.get(0).unwrap().parse::<u16>().unwrap(); // 3
            if color_count > *color_limits.get(color_data[1]).unwrap() {
                return false;
            }
        }
    }

    true
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
}
