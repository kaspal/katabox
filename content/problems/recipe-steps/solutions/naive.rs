pub fn step_order(n: i64, before: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·(n + m)): before every step, rescan all rules to see which steps
    // are still blocked, then scan all steps for the lowest ready one.
    let n = n as usize;
    let mut finished = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for _ in 0..n {
        let mut blocked = vec![false; n];
        for e in &before {
            if !finished[e[0] as usize] {
                blocked[e[1] as usize] = true;
            }
        }
        match (0..n).find(|&s| !finished[s] && !blocked[s]) {
            Some(s) => {
                finished[s] = true;
                order.push(s as i64);
            }
            None => return Vec::new(),
        }
    }
    order
}
