pub fn users_with_all(users: Vec<i64>, queries: Vec<i64>) -> Vec<i64> {
    let top = users.iter().chain(&queries).copied().max().unwrap_or(1).max(1) as usize;
    let bits = usize::BITS - top.leading_zeros();
    let size = 1usize << bits;
    let mut count = vec![0i64; size];
    for &u in &users {
        count[u as usize] += 1;
    }
    for b in 0..bits {
        let bit = 1usize << b;
        for m in 0..size {
            if m & bit == 0 {
                count[m] += count[m | bit];
            }
        }
    }
    queries.iter().map(|&q| count[q as usize]).collect()
}
