use aoc_2023_rs::trebuchet;

#[test]
fn sum_digits_returns_expected_result_for_example() {
    let result = trebuchet::sum_digits("./src/day_1/example.txt");
    assert_eq!(result, 142);
}

#[test]
fn sum_digits_returns_expected_result_for_data() {
    let result = trebuchet::sum_digits("./src/day_1/data.txt");
    assert_eq!(result, 55477);
}
