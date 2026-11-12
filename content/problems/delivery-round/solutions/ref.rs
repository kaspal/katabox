pub fn shortest_round(dist: Vec<Vec<i64>>) -> i64 {
    let n = dist.len();
    if n == 1 {
        return 0;
    }
    let full = (1usize << n) - 1;
    let mut best = vec![i64::MAX; (full + 1) * n];
    for last in 0..n {
        best[full * n + last] = dist[last][0];
    }
    // Only masks containing the depot (odd masks), from the fullest down.
    let mut mask = full - 2;
    loop {
        for last in 0..n {
            if mask >> last & 1 == 0 {
                continue;
            }
            let mut b = i64::MAX;
            for j in 1..n {
                if mask >> j & 1 == 0 {
                    b = b.min(dist[last][j] + best[(mask | 1 << j) * n + j]);
                }
            }
            best[mask * n + last] = b;
        }
        if mask == 1 {
            break;
        }
        mask -= 2;
    }
    best[n]
}
