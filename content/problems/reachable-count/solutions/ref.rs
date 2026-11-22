pub fn reachable_count(n: i64, doors: Vec<Vec<i64>>, start: i64) -> i64 {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for d in &doors {
        adj[d[0] as usize].push(d[1] as usize);
        adj[d[1] as usize].push(d[0] as usize);
    }
    let mut seen = vec![false; n];
    seen[start as usize] = true;
    let mut queue = vec![start as usize];
    let mut head = 0;
    while head < queue.len() {
        let v = queue[head];
        head += 1;
        for &u in &adj[v] {
            if !seen[u] {
                seen[u] = true;
                queue.push(u);
            }
        }
    }
    queue.len() as i64
}
