pub fn toll_refunds(n: i64, roads: Vec<Vec<i64>>, start: i64) -> Vec<i64> {
    const INF: i64 = i64::MAX / 4;
    let n = n as usize;
    let mut cost = vec![INF; n];
    cost[start as usize] = 0;
    for _ in 1..n {
        let mut changed = false;
        for r in &roads {
            let (a, b, c) = (r[0] as usize, r[1] as usize, r[2]);
            if cost[a] != INF && cost[a] + c < cost[b] {
                cost[b] = cost[a] + c;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for r in &roads {
        let (a, b, c) = (r[0] as usize, r[1] as usize, r[2]);
        if cost[a] != INF && cost[a] + c < cost[b] {
            return Vec::new();
        }
    }
    cost.into_iter().map(|x| if x == INF { 1_000_000_000 } else { x }).collect()
}
