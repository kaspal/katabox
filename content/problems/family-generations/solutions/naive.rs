pub fn generations(parent: Vec<i64>) -> i64 {
    // Correct but O(n · depth): walk up from every person to the founder.
    let mut best = 0;
    for i in 0..parent.len() {
        let (mut steps, mut p) = (1, parent[i]);
        while p >= 0 {
            steps += 1;
            p = parent[p as usize];
        }
        best = best.max(steps);
    }
    best
}
