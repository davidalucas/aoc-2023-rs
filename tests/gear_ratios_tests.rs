use aoc_2023_rs::gear_ratios::{calc_sum_of_gear_ratios, calc_sum_of_part_numbers};

#[test]
fn calc_sum_of_part_numbers_works_for_example_data() {
    let result = calc_sum_of_part_numbers("./src/day_3/example.txt");
    assert_eq!(result, 4361);
}

#[test]
fn calc_sum_of_part_numbers_works_for_real_data() {
    let result = calc_sum_of_part_numbers("./src/day_3/data.txt");
    assert_eq!(result, 544664);
}

#[test]
fn calc_sum_of_gear_ratios_works_for_example_data() {
    let result = calc_sum_of_gear_ratios("./src/day_3/example.txt");
    assert_eq!(result, 467835);
}

#[test]
fn calc_sum_of_gear_ratios_works_for_real_data() {
    let result = calc_sum_of_gear_ratios("./src/day_3/data.txt");
    assert_eq!(result, 84495585);
}
