pub fn within_fixes(typed: String, meant: String, k: i64) -> bool {
    let (t, m) = (typed.as_bytes(), meant.as_bytes());
    let (n, mm, k) = (t.len() as i64, m.len() as i64, k);
    if (n - mm).abs() > k {
        return false;
    }
    let big = k + 1;
    let w = (2 * k + 1) as usize;
    let mut prev = vec![big; w];
    for j in 0..=mm.min(k) {
        prev[(j + k) as usize] = j;
    }
    for i in 1..=n {
        let mut cur = vec![big; w];
        let (lo, hi) = ((i - k).max(0), mm.min(i + k));
        for j in lo..=hi {
            let d = (j - i + k) as usize;
            if j == 0 {
                cur[d] = i.min(big);
                continue;
            }
            let mut best = prev[d] + i64::from(t[(i - 1) as usize] != m[(j - 1) as usize]);
            if d + 1 < w {
                best = best.min(prev[d + 1] + 1);
            }
            if d >= 1 {
                best = best.min(cur[d - 1] + 1);
            }
            cur[d] = best.min(big);
        }
        prev = cur;
    }
    prev[(mm - n + k) as usize] <= k
}
