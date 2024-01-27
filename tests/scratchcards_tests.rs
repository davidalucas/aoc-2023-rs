use aoc_2023_rs::scratchcards::{sum_total_scratchcards, sum_total_winnings};

#[test]
fn sum_total_winnings_works_for_example_data() {
    let result = sum_total_winnings("./data/day_4/example.txt");
    assert_eq!(result, 13);
}

#[test]
fn sum_total_winnings_works_for_real_data() {
    let result = sum_total_winnings("./data/day_4/data.txt");
    assert_eq!(result, 23235);
}

#[test]
fn sum_total_scratchcards_works_for_example_data() {
    let result = sum_total_scratchcards("./data/day_4/example.txt");
    assert_eq!(result, 30);
}

#[test]
fn sum_total_scratchcards_works_for_real_data() {
    let result = sum_total_scratchcards("./data/day_4/data.txt");
    assert_eq!(result, 5920640);
}
