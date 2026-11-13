pub fn shared_melody(first: Vec<i64>, second: Vec<i64>) -> i64 {
    let m = second.len();
    let mut prev = vec![0i64; m + 1];
    let mut cur = vec![0i64; m + 1];
    for &x in &first {
        for j in 1..=m {
            cur[j] = if x == second[j - 1] { prev[j - 1] + 1 } else { prev[j].max(cur[j - 1]) };
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[m]
}
