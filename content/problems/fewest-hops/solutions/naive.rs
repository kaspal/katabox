pub fn fewest_hops(n: i64, links: Vec<Vec<i64>>) -> i64 {
    // Correct but O(n·m): relax every link in both directions until nothing improves.
    let n = n as usize;
    let inf = n as i64 + 1;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for l in &links {
            let (a, b) = (l[0] as usize, l[1] as usize);
            if dist[a] + 1 < dist[b] {
                dist[b] = dist[a] + 1;
                changed = true;
            }
            if dist[b] + 1 < dist[a] {
                dist[a] = dist[b] + 1;
                changed = true;
            }
        }
    }
    if dist[n - 1] < inf { dist[n - 1] } else { -1 }
}
