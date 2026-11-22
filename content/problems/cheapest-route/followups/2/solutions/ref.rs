use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn cheapest_with_pass(n: i64, roads: Vec<Vec<i64>>, start: i64, goal: i64) -> i64 {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for r in &roads {
        adj[r[0] as usize].push((r[1] as usize, r[2]));
        adj[r[1] as usize].push((r[0] as usize, r[2]));
    }
    let inf = i64::MAX;
    let mut dist = vec![inf; 2 * n];
    let s0 = start as usize * 2;
    dist[s0] = 0;
    let mut heap = BinaryHeap::from([Reverse((0i64, s0))]);
    while let Some(Reverse((d, s))) = heap.pop() {
        if d > dist[s] {
            continue;
        }
        let (u, used) = (s / 2, s % 2);
        for &(v, t) in &adj[u] {
            if d + t < dist[v * 2 + used] {
                dist[v * 2 + used] = d + t;
                heap.push(Reverse((d + t, v * 2 + used)));
            }
            if used == 0 && d < dist[v * 2 + 1] {
                dist[v * 2 + 1] = d;
                heap.push(Reverse((d, v * 2 + 1)));
            }
        }
    }
    let g = goal as usize * 2;
    let best = dist[g].min(dist[g + 1]);
    if best == inf { -1 } else { best }
}
