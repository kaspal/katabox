pub fn cut_cost(length: i64, marks: Vec<i64>) -> i64 {
    let mut p = vec![0];
    let mut sorted = marks;
    sorted.sort();
    p.extend(sorted);
    p.push(length);
    let m = p.len();
    let mut cost = vec![vec![0i64; m]; m];
    for gap in 2..m {
        for i in 0..m - gap {
            let j = i + gap;
            let best = (i + 1..j).map(|k| cost[i][k] + cost[k][j]).min().unwrap();
            cost[i][j] = p[j] - p[i] + best;
        }
    }
    cost[0][m - 1]
}
