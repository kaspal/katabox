pub fn cheapest_within(n: i64, roads: Vec<Vec<i64>>, start: i64, goal: i64, k: i64) -> i64 {
    let inf = i64::MAX;
    let mut best = vec![inf; n as usize];
    best[start as usize] = 0;
    for _ in 0..k {
        let mut nxt = best.clone();
        let mut changed = false;
        for r in &roads {
            let (a, b, t) = (r[0] as usize, r[1] as usize, r[2]);
            if best[a] != inf && best[a] + t < nxt[b] {
                nxt[b] = best[a] + t;
                changed = true;
            }
            if best[b] != inf && best[b] + t < nxt[a] {
                nxt[a] = best[b] + t;
                changed = true;
            }
        }
        best = nxt;
        if !changed {
            break;
        }
    }
    if best[goal as usize] == inf { -1 } else { best[goal as usize] }
}
