fn reach(start: usize, graph: &[Vec<usize>]) -> Vec<bool> {
    let mut seen = vec![false; graph.len()];
    seen[start] = true;
    let mut stack = vec![start];
    while let Some(x) = stack.pop() {
        for &y in &graph[x] {
            if !seen[y] {
                seen[y] = true;
                stack.push(y);
            }
        }
    }
    seen
}

pub fn mutual_reach(n: i64, links: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·(n + m)): one search per user, forwards and backwards.
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    let mut radj = vec![Vec::new(); n];
    for e in &links {
        adj[e[0] as usize].push(e[1] as usize);
        radj[e[1] as usize].push(e[0] as usize);
    }
    let mut assigned = vec![false; n];
    let mut sizes = Vec::new();
    for s in 0..n {
        if assigned[s] {
            continue;
        }
        let (fwd, back) = (reach(s, &adj), reach(s, &radj));
        let mut count = 0;
        for v in 0..n {
            if fwd[v] && back[v] {
                assigned[v] = true;
                count += 1;
            }
        }
        sizes.push(count);
    }
    sizes.sort();
    sizes
}
