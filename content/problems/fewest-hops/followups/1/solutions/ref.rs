pub fn count_routes(n: i64, links: Vec<Vec<i64>>) -> i64 {
    const MOD: i64 = 1_000_003;
    let n = n as usize;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for l in &links {
        let (a, b) = (l[0] as usize, l[1] as usize);
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut dist = vec![-1i64; n];
    let mut ways = vec![0i64; n];
    dist[0] = 0;
    ways[0] = 1;
    let mut queue = vec![0usize];
    let mut head = 0;
    while head < queue.len() {
        let u = queue[head];
        head += 1;
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                ways[v] = ways[u];
                queue.push(v);
            } else if dist[v] == dist[u] + 1 {
                ways[v] = (ways[v] + ways[u]) % MOD;
            }
        }
    }
    ways[n - 1] % MOD
}
