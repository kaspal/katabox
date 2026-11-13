pub fn largest_poster(panels: Vec<i64>) -> i64 {
    // Correct but O(n²): every left end, with a running minimum to the right.
    let mut best = 0;
    for i in 0..panels.len() {
        let mut low = panels[i];
        for j in i..panels.len() {
            low = low.min(panels[j]);
            best = best.max(low * (j - i + 1) as i64);
        }
    }
    best
}
