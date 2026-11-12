use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn cheapest_route(n: i64, roads: Vec<Vec<i64>>, start: i64, goal: i64) -> i64 {
    let n = n as usize;
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for r in &roads {
        let (a, b) = (r[0] as usize, r[1] as usize);
        adj[a].push((b, r[2]));
        adj[b].push((a, r[2]));
    }
    let mut dist = vec![i64::MAX; n];
    let (start, goal) = (start as usize, goal as usize);
    dist[start] = 0;
    let mut heap = BinaryHeap::from([Reverse((0i64, start))]);
    while let Some(Reverse((d, t))) = heap.pop() {
        if d > dist[t] {
            continue;
        }
        if t == goal {
            return d;
        }
        for &(u, toll) in &adj[t] {
            if d + toll < dist[u] {
                dist[u] = d + toll;
                heap.push(Reverse((d + toll, u)));
            }
        }
    }
    -1
}
