use aoc_2023_rs::scratchcards::sum_total_winnings;

#[test]
fn sum_total_winnings_works_for_example_data() {
    let result = sum_total_winnings("./src/day_4/example.txt");
    assert_eq!(result, 13);
}

#[test]
fn sum_total_winnings_works_for_real_data() {
    let result = sum_total_winnings("./src/day_4/data.txt");
    assert_eq!(result, 23235);
}