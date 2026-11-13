pub fn ferry_times(n: i64, routes: Vec<Vec<i64>>, queries: Vec<Vec<i64>>) -> Vec<i64> {
    const INF: i64 = i64::MAX / 4;
    let n = n as usize;
    let mut dist = vec![vec![INF; n]; n];
    for (i, row) in dist.iter_mut().enumerate() {
        row[i] = 0;
    }
    for r in &routes {
        let (a, b, w) = (r[0] as usize, r[1] as usize, r[2]);
        if w < dist[a][b] {
            dist[a][b] = w;
            dist[b][a] = w;
        }
    }
    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik == INF {
                continue;
            }
            for j in 0..n {
                let via = dik + dist[k][j];
                if via < dist[i][j] {
                    dist[i][j] = via;
                }
            }
        }
    }
    queries
        .iter()
        .map(|q| {
            let d = dist[q[0] as usize][q[1] as usize];
            if d >= INF { -1 } else { d }
        })
        .collect()
}
