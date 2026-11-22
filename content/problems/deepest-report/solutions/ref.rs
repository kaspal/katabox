pub fn deepest_report(manager: Vec<i64>) -> i64 {
    let n = manager.len();
    let mut depth = vec![0i64; n];
    let mut best = 0;
    for v in 0..n {
        let mut path = Vec::new();
        let mut u = v as i64;
        while u != -1 && depth[u as usize] == 0 {
            path.push(u as usize);
            u = manager[u as usize];
        }
        let mut d = if u == -1 { 0 } else { depth[u as usize] };
        for &w in path.iter().rev() {
            d += 1;
            depth[w] = d;
        }
        best = best.max(depth[v]);
    }
    best
}
