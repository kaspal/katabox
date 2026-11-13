pub fn trip_totals(miles: Vec<i64>, trips: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·q): walk every trip's days.
    trips
        .iter()
        .map(|t| miles[t[0] as usize..=t[1] as usize].iter().sum())
        .collect()
}
