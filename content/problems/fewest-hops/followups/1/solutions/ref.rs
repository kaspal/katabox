use std::collections::VecDeque;

pub fn hops_to_all(n: i64, links: Vec<Vec<i64>>) -> Vec<i64> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for l in &links {
        adj[l[0] as usize].push(l[1] as usize);
        adj[l[1] as usize].push(l[0] as usize);
    }
    let mut dist = vec![-1i64; n];
    dist[0] = 0;
    let mut q = VecDeque::from([0usize]);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }
    dist
}
