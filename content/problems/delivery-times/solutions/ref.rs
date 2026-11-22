pub fn delivery_times(n: i64, roads: Vec<Vec<i64>>, start: i64) -> Vec<i64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for r in &roads {
        adj[r[0] as usize].push((r[1] as usize, r[2]));
        adj[r[1] as usize].push((r[0] as usize, r[2]));
    }
    let mut dist = vec![-1i64; n];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, start as usize)));
    while let Some(Reverse((d, v))) = heap.pop() {
        if dist[v] != -1 {
            continue;
        }
        dist[v] = d;
        for &(u, w) in &adj[v] {
            if dist[u] == -1 {
                heap.push(Reverse((d + w, u)));
            }
        }
    }
    dist
}
