pub fn days_until_warmer_around(temps: Vec<i64>) -> Vec<i64> {
    let n = temps.len();
    let mut wait = vec![0i64; n];
    let mut stack: Vec<usize> = Vec::new();
    for k in 0..2 * n {
        let i = k % n;
        while let Some(&d) = stack.last() {
            if temps[d] >= temps[i] {
                break;
            }
            stack.pop();
            let w = (i + n - d) % n;
            wait[d] = if w == 0 { n as i64 } else { w as i64 };
        }
        if k < n {
            stack.push(i);
        }
    }
    wait
}
