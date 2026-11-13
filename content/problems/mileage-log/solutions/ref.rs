pub fn trip_totals(miles: Vec<i64>, trips: Vec<Vec<i64>>) -> Vec<i64> {
    let mut prefix = vec![0i64; miles.len() + 1];
    for (i, m) in miles.iter().enumerate() {
        prefix[i + 1] = prefix[i] + m;
    }
    trips
        .iter()
        .map(|t| prefix[t[1] as usize + 1] - prefix[t[0] as usize])
        .collect()
}
