pub fn mirror_cuts(text: String) -> i64 {
    let t = text.as_bytes();
    let n = t.len() as i64;
    let mut cuts: Vec<i64> = (0..=n).map(|i| i - 1).collect();
    for c in 0..n {
        for (mut l, mut r) in [(c, c), (c, c + 1)] {
            while l >= 0 && r < n && t[l as usize] == t[r as usize] {
                let cand = cuts[l as usize] + 1;
                if cand < cuts[(r + 1) as usize] {
                    cuts[(r + 1) as usize] = cand;
                }
                l -= 1;
                r += 1;
            }
        }
    }
    cuts[n as usize]
}
