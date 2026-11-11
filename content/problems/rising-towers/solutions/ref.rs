pub fn longest_rise(heights: Vec<i64>) -> i64 {
    let mut tails: Vec<i64> = Vec::new();
    for h in heights {
        let p = tails.partition_point(|&t| t < h);
        if p == tails.len() {
            tails.push(h);
        } else {
            tails[p] = h;
        }
    }
    tails.len() as i64
}
