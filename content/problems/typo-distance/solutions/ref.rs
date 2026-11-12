pub fn typo_distance(typed: String, meant: String) -> i64 {
    let (a, b) = (typed.as_bytes(), meant.as_bytes());
    let m = b.len();
    let mut prev: Vec<i64> = (0..=m as i64).collect();
    let mut cur = vec![0i64; m + 1];
    for i in 1..=a.len() {
        cur[0] = i as i64;
        for j in 1..=m {
            cur[j] = if a[i - 1] == b[j - 1] {
                prev[j - 1]
            } else {
                1 + prev[j].min(cur[j - 1]).min(prev[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[m]
}
