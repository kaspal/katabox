pub fn count_below(codes: Vec<i64>, queries: Vec<i64>) -> Vec<i64> {
    queries
        .iter()
        .map(|&x| {
            let (mut lo, mut hi) = (0usize, codes.len());
            while lo < hi {
                let mid = (lo + hi) / 2;
                if codes[mid] < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo as i64
        })
        .collect()
}
