pub fn cheapest_route(n: i64, roads: Vec<Vec<i64>>, start: i64, goal: i64) -> i64 {
    // Correct but O(n·m): Bellman-Ford, relaxing every road round after round until
    // nothing changes.
    let inf = i64::MAX / 4;
    let mut dist = vec![inf; n as usize];
    dist[start as usize] = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for r in &roads {
            let (a, b, toll) = (r[0] as usize, r[1] as usize, r[2]);
            if dist[a] + toll < dist[b] {
                dist[b] = dist[a] + toll;
                changed = true;
            }
            if dist[b] + toll < dist[a] {
                dist[a] = dist[b] + toll;
                changed = true;
            }
        }
    }
    if dist[goal as usize] >= inf { -1 } else { dist[goal as usize] }
}
