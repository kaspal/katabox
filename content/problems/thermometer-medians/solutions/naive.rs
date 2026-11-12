pub fn running_medians(readings: Vec<i64>) -> Vec<f64> {
    // Correct but O(n² log n): sort a copy of every prefix.
    let mut out = Vec::with_capacity(readings.len());
    for i in 1..=readings.len() {
        let mut s = readings[..i].to_vec();
        s.sort_unstable();
        out.push(if i % 2 == 1 {
            s[i / 2] as f64
        } else {
            (s[i / 2 - 1] + s[i / 2]) as f64 / 2.0
        });
    }
    out
}
