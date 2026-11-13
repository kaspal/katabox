pub fn kth_gap(posts: Vec<i64>, k: i64) -> i64 {
    // Correct but O(n² + W): count every gap in a table indexed by its length.
    let lo = *posts.iter().min().unwrap();
    let hi = *posts.iter().max().unwrap();
    let mut table = vec![0i64; (hi - lo + 1) as usize];
    for i in 0..posts.len() {
        for j in i + 1..posts.len() {
            table[(posts[i] - posts[j]).unsigned_abs() as usize] += 1;
        }
    }
    let mut k = k;
    for (gap, &c) in table.iter().enumerate() {
        k -= c;
        if k <= 0 {
            return gap as i64;
        }
    }
    -1
}
