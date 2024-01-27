use aoc_2023_rs::boat_race::calc_charging_times;

#[test]
fn calc_charging_times_works_for_example_data() {
    let example_data = vec![(7, 9), (15, 40), (30, 200)];
    let result = example_data
        .iter()
        .map(|d| {
            let times = calc_charging_times(d.0, d.1);
            times.1 - times.0 + 1
        })
        .reduce(|a, b| a * b)
        .unwrap();

    assert_eq!(result, 288);
}

#[test]
fn calc_charging_times_works_for_real_data() {
    let real_data = vec![(46, 358), (68, 1054), (98, 1807), (66, 1080)];
    let result = real_data
        .iter()
        .map(|d| {
            let times = calc_charging_times(d.0, d.1);
            times.1 - times.0 + 1
        })
        .reduce(|a, b| a * b)
        .unwrap();

    assert_eq!(result, 138915);
}
