pub fn best_pack_huge(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> i64 {
    let total: usize = values.iter().sum::<i64>() as usize;
    let inf = i64::MAX;
    let mut lightest = vec![inf; total + 1];
    lightest[0] = 0;
    for (w, v) in weights.iter().zip(values.iter()) {
        let v = *v as usize;
        for t in (v..=total).rev() {
            if lightest[t - v] != inf && lightest[t - v] + w < lightest[t] {
                lightest[t] = lightest[t - v] + w;
            }
        }
    }
    (0..=total).rev().find(|&t| lightest[t] <= capacity).unwrap_or(0) as i64
}
