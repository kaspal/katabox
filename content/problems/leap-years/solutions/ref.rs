pub fn leap_years(start: i64, end: i64) -> i64 {
    (start..=end)
        .filter(|y| y % 4 == 0 && (y % 100 != 0 || y % 400 == 0))
        .count() as i64
}
