pub fn temperature_swing(readings: Vec<i64>) -> i64 {
    let mut warmest = readings[0];
    let mut coldest = readings[0];
    for &r in &readings {
        warmest = warmest.max(r);
        coldest = coldest.min(r);
    }
    warmest - coldest
}
