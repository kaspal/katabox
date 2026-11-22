pub fn best_pack_copies(weights: Vec<i64>, values: Vec<i64>, counts: Vec<i64>, capacity: i64) -> i64 {
    let capacity = capacity as usize;
    let mut best = vec![0i64; capacity + 1];
    for i in 0..weights.len() {
        let mut c = counts[i];
        let mut k = 1;
        while c > 0 {
            let take = k.min(c);
            let bw = (weights[i] * take) as usize;
            let bv = values[i] * take;
            if bw <= capacity {
                for cap in (bw..=capacity).rev() {
                    if best[cap - bw] + bv > best[cap] {
                        best[cap] = best[cap - bw] + bv;
                    }
                }
            }
            c -= take;
            k *= 2;
        }
    }
    best[capacity]
}
