pub fn min_trips(weights: Vec<i64>, limit: i64) -> i64 {
    // Correct but O(n²): heaviest first, each looks through everyone left for the
    // heaviest companion that still fits.
    let mut w = weights;
    w.sort_unstable_by(|a, b| b.cmp(a));
    let n = w.len();
    let mut used = vec![false; n];
    let mut trips = 0;
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[i] = true;
        for j in i + 1..n {
            if !used[j] && w[i] + w[j] <= limit {
                used[j] = true;
                break;
            }
        }
        trips += 1;
    }
    trips
}
