pub fn days_until_warmer(temps: Vec<i64>) -> Vec<i64> {
    // Correct but O(n²) on long flat or cold spells: scan forward from every day.
    let n = temps.len();
    let mut wait = vec![0i64; n];
    for i in 0..n {
        for j in i + 1..n {
            if temps[j] > temps[i] {
                wait[i] = (j - i) as i64;
                break;
            }
        }
    }
    wait
}
