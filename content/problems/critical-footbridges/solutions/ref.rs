pub fn critical_bridges(n: i64, bridges: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for (idx, e) in bridges.iter().enumerate() {
        adj[e[0] as usize].push((e[1] as usize, idx));
        adj[e[1] as usize].push((e[0] as usize, idx));
    }
    let mut tin = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut timer = 0;
    let mut out = Vec::new();
    for s in 0..n {
        if tin[s] != usize::MAX {
            continue;
        }
        tin[s] = timer;
        low[s] = timer;
        timer += 1;
        let mut stack: Vec<(usize, usize, usize)> = vec![(s, 0, usize::MAX)];
        while let Some(&(v, i, via)) = stack.last() {
            if i < adj[v].len() {
                stack.last_mut().unwrap().1 += 1;
                let (u, idx) = adj[v][i];
                if idx == via {
                    continue;
                }
                if tin[u] == usize::MAX {
                    tin[u] = timer;
                    low[u] = timer;
                    timer += 1;
                    stack.push((u, 0, idx));
                } else {
                    low[v] = low[v].min(tin[u]);
                }
            } else {
                stack.pop();
                if let Some(&(p, _, _)) = stack.last() {
                    low[p] = low[p].min(low[v]);
                    if low[v] > tin[p] {
                        out.push(vec![p.min(v) as i64, p.max(v) as i64]);
                    }
                }
            }
        }
    }
    out.sort();
    out
}
