pub fn best_programme(talks: Vec<Vec<i64>>) -> i64 {
    let mut order = talks;
    order.sort_by_key(|t| t[1]);
    let n = order.len();
    let mut best = vec![0i64; n + 1];
    for i in 0..n {
        let (s, score) = (order[i][0], order[i][2]);
        let j = order[..i].partition_point(|t| t[1] <= s);
        best[i + 1] = best[i].max(best[j] + score);
    }
    best[n]
}
