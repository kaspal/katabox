pub fn rising_run(heights: Vec<i64>) -> i64 {
    let mut tails: Vec<i64> = Vec::new();
    for h in heights {
        let i = tails.partition_point(|&t| t < h);
        if i == tails.len() {
            tails.push(h);
        } else {
            tails[i] = h;
        }
    }
    tails.len() as i64
}
