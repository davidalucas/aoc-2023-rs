pub fn calc_charging_times(time: i64, distance: i64) -> (i64, i64) {
    let time = time as f64;
    let distance = distance as f64;
    let lowest = (time - f64::sqrt(time.powf(2.0) - 4.0 * distance)) / 2.0;
    let highest = (time + f64::sqrt(time.powf(2.0) - 4.0 * distance)) / 2.0;

    (lowest.floor() as i64 + 1, highest.ceil() as i64 - 1)
}
