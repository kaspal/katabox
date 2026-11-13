fn go(weights: &[i64], values: &[i64], i: usize, room: i64) -> i64 {
    if i == weights.len() {
        return 0;
    }
    let mut best = go(weights, values, i + 1, room);
    if weights[i] <= room {
        best = best.max(values[i] + go(weights, values, i + 1, room - weights[i]));
    }
    best
}

pub fn best_pack(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> i64 {
    // Correct but O(2ⁿ): take or skip each item recursively, without remembering anything.
    go(&weights, &values, 0, capacity)
}
