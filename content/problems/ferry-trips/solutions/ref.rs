pub fn min_trips(weights: Vec<i64>, limit: i64) -> i64 {
    let mut w = weights;
    w.sort_unstable();
    let (mut lo, mut hi) = (0i64, w.len() as i64 - 1);
    let mut trips = 0;
    while lo <= hi {
        if w[lo as usize] + w[hi as usize] <= limit {
            lo += 1;
        }
        hi -= 1;
        trips += 1;
    }
    trips
}
