use aoc_2023_rs::trebuchet;

#[test]
fn sum_digits_returns_expected_result_for_example() {
    let result = trebuchet::sum_digits("./data/day_1/example1.txt");
    assert_eq!(result, 142);
}

#[test]
fn sum_digits_returns_expected_result_for_data() {
    let result = trebuchet::sum_digits("./data/day_1/data.txt");
    assert_eq!(result, 55477);
}

#[test]
fn sum_digits_enhanced_returns_expected_result_for_example() {
    let result = trebuchet::sum_digits_enhanced("./data/day_1/example2.txt");
    assert_eq!(result, 281);
}

#[test]
fn sum_digits_enhanced_returns_expected_result_for_data() {
    let result = trebuchet::sum_digits_enhanced("./data/day_1/data.txt");
    assert_eq!(result, 54431);
}
