use std::collections::HashMap;

use aoc_2023_rs::cube_game::sum_valid_games;

#[test]
fn sum_valid_games_works_for_example_data() {
    let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let result = sum_valid_games("./src/day_2/example.txt", color_limits).unwrap();
    assert_eq!(result, 8);
}

#[test]
fn sum_valid_games_works_for_real_data() {
    let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let result = sum_valid_games("./src/day_2/data.txt", color_limits).unwrap();
    assert_eq!(result, 2727);
}
