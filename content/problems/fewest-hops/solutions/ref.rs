pub fn fewest_hops(n: i64, links: Vec<Vec<i64>>) -> i64 {
    let n = n as usize;
    let mut near: Vec<Vec<usize>> = vec![Vec::new(); n];
    for l in &links {
        near[l[0] as usize].push(l[1] as usize);
        near[l[1] as usize].push(l[0] as usize);
    }
    let mut dist = vec![-1i64; n];
    dist[0] = 0;
    let mut queue = vec![0usize];
    let mut h = 0;
    while h < queue.len() {
        let v = queue[h];
        h += 1;
        for &u in &near[v] {
            if dist[u] == -1 {
                dist[u] = dist[v] + 1;
                queue.push(u);
            }
        }
    }
    dist[n - 1]
}
