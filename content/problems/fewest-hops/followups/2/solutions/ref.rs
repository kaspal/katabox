use std::collections::VecDeque;

pub fn hops_one_way(n: i64, links: Vec<Vec<i64>>) -> i64 {
    let n = n as usize;
    let mut fwd = vec![Vec::new(); n];
    let mut back = vec![Vec::new(); n];
    for l in &links {
        fwd[l[0] as usize].push(l[1] as usize);
        back[l[1] as usize].push(l[0] as usize);
    }
    let mut dist = vec![-1i64; 2 * n];
    dist[0] = 0;
    let mut q = VecDeque::from([0usize]);
    while let Some(s) = q.pop_front() {
        let (u, f, d) = (s / 2, s % 2, dist[s]);
        for &v in &fwd[u] {
            if dist[v * 2 + f] == -1 {
                dist[v * 2 + f] = d + 1;
                q.push_back(v * 2 + f);
            }
        }
        if f == 0 {
            for &v in &back[u] {
                if dist[v * 2 + 1] == -1 {
                    dist[v * 2 + 1] = d + 1;
                    q.push_back(v * 2 + 1);
                }
            }
        }
    }
    match (dist[(n - 1) * 2], dist[(n - 1) * 2 + 1]) {
        (-1, b) => b,
        (a, -1) => a,
        (a, b) => a.min(b),
    }
}
