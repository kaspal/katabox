pub fn kth_in_window(scores: Vec<i64>, w: i64, k: i64) -> Vec<i64> {
    let (w, n) = (w as usize, scores.len());
    let mut count = [0i64; 301];
    let mut out = Vec::new();
    for i in 0..n {
        count[scores[i] as usize] += 1;
        if i >= w {
            count[scores[i - w] as usize] -= 1;
        }
        if i + 1 >= w {
            let mut left = k;
            let mut v = 300usize;
            while left > count[v] {
                left -= count[v];
                v -= 1;
            }
            out.push(v as i64);
        }
    }
    out
}
