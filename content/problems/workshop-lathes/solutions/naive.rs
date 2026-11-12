pub fn finish_time(jobs: Vec<i64>, lathes: i64) -> i64 {
    // Correct but O(n·m): look through every lathe for the one free first.
    let mut free = vec![0i64; lathes as usize];
    let mut latest = 0;
    for job in jobs {
        let mut best = 0;
        for k in 1..free.len() {
            if free[k] < free[best] {
                best = k;
            }
        }
        free[best] += job;
        latest = latest.max(free[best]);
    }
    latest
}
