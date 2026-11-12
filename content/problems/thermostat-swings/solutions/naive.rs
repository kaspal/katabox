pub fn window_swings(readings: Vec<i64>, k: i64) -> Vec<i64> {
    // Correct but O(n·k): scan every window for its highest and lowest reading.
    let k = k as usize;
    readings
        .windows(k)
        .map(|w| {
            let mut hi = w[0];
            let mut lo = w[0];
            for &r in &w[1..] {
                hi = hi.max(r);
                lo = lo.min(r);
            }
            hi - lo
        })
        .collect()
}
