pub fn longest_piece(ribbons: Vec<i64>, pieces: i64) -> i64 {
    let count = |len: i64| -> i64 { ribbons.iter().map(|r| r / len).sum() };
    if count(1) < pieces {
        return 0;
    }
    let (mut lo, mut hi) = (1, *ribbons.iter().max().unwrap());
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if count(mid) >= pieces {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}
