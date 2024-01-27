pub fn calc_charging_times(time: i32, distance: i32) -> (i32, i32) {
    let time = time as f32;
    let distance = distance as f32;
    let lowest = (time - f32::sqrt(time.powf(2.0) - 4.0 * distance)) / 2.0;
    let highest = (time + f32::sqrt(time.powf(2.0) - 4.0 * distance)) / 2.0;

    (lowest.floor() as i32 + 1, highest.ceil() as i32 - 1)
}
