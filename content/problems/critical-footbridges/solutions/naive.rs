pub fn critical_bridges(n: i64, bridges: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    // Correct but O(m·(n + m)): close each footbridge, then check whether its two ends
    // still reach each other.
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for (idx, e) in bridges.iter().enumerate() {
        adj[e[0] as usize].push((e[1] as usize, idx));
        adj[e[1] as usize].push((e[0] as usize, idx));
    }
    let mut out = Vec::new();
    for (idx, e) in bridges.iter().enumerate() {
        let (a, b) = (e[0] as usize, e[1] as usize);
        let mut seen = vec![false; n];
        seen[a] = true;
        let mut stack = vec![a];
        while let Some(x) = stack.pop() {
            if seen[b] {
                break;
            }
            for &(y, j) in &adj[x] {
                if j != idx && !seen[y] {
                    seen[y] = true;
                    stack.push(y);
                }
            }
        }
        if !seen[b] {
            out.push(vec![a.min(b) as i64, a.max(b) as i64]);
        }
    }
    out.sort();
    out
}
