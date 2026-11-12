pub fn days_until_warmer(temps: Vec<i64>) -> Vec<i64> {
    let mut wait = vec![0i64; temps.len()];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..temps.len() {
        while let Some(&j) = stack.last() {
            if temps[j] >= temps[i] {
                break;
            }
            stack.pop();
            wait[j] = (i - j) as i64;
        }
        stack.push(i);
    }
    wait
}
