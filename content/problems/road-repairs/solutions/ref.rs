pub fn road_repairs(length: i64, repairs: Vec<Vec<i64>>) -> Vec<i64> {
    let n = length as usize;
    let mut change = vec![0i64; n + 1];
    for r in &repairs {
        change[r[0] as usize] += r[2];
        change[r[1] as usize + 1] -= r[2];
    }
    let mut running = 0;
    change[..n]
        .iter()
        .map(|c| {
            running += c;
            running
        })
        .collect()
}
