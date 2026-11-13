pub fn can_cross(jumps: Vec<i64>) -> bool {
    // Correct but O(n · max jump): a search that tries every jump length from every
    // reachable stone.
    let n = jumps.len();
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut stack = vec![0usize];
    while let Some(i) = stack.pop() {
        let end = n.min(i + jumps[i] as usize + 1);
        for k in i + 1..end {
            if !seen[k] {
                seen[k] = true;
                stack.push(k);
            }
        }
    }
    seen[n - 1]
}
