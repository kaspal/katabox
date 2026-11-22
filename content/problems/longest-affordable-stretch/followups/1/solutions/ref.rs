pub fn longest_with_refunds(costs: Vec<i64>, budget: i64) -> i64 {
    let mut starts: Vec<i64> = vec![0];
    let mut where_: Vec<usize> = vec![0];
    let (mut prefix, mut best) = (0i64, 0usize);
    for (idx, c) in costs.iter().enumerate() {
        let j = idx + 1;
        prefix += c;
        let k = starts.partition_point(|&s| s < prefix - budget);
        if k < starts.len() {
            best = best.max(j - where_[k]);
        }
        if prefix > *starts.last().unwrap() {
            starts.push(prefix);
            where_.push(j);
        }
    }
    best as i64
}
