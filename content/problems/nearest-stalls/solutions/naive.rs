pub fn nearest_stalls(stalls: Vec<Vec<i64>>, k: i64) -> Vec<i64> {
    // Correct but O(n·k): select the nearest remaining stall k times.
    let dist: Vec<i64> = stalls.iter().map(|s| s[0] * s[0] + s[1] * s[1]).collect();
    let mut taken = vec![false; dist.len()];
    let mut out = Vec::new();
    for _ in 0..k {
        let mut best = usize::MAX;
        for i in 0..dist.len() {
            if !taken[i] && (best == usize::MAX || dist[i] < dist[best]) {
                best = i;
            }
        }
        taken[best] = true;
        out.push(best as i64);
    }
    out
}
