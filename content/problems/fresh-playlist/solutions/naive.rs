use std::collections::HashSet;

pub fn fresh_stretch(plays: Vec<i64>) -> i64 {
    // Correct but O(n · answer): grow a stretch from every start until a repeat.
    let mut best = 0;
    let mut seen = HashSet::new();
    for i in 0..plays.len() {
        seen.clear();
        for &x in &plays[i..] {
            if !seen.insert(x) {
                break;
            }
        }
        best = best.max(seen.len());
    }
    best as i64
}
