pub fn longest_run(values: Vec<i64>) -> i64 {
    let (mut best, mut run) = (0, 1);
    for i in 1..values.len() {
        if values[i] == values[i - 1] {
            run += 1;
        } else {
            best = best.max(run);
            run = 1;
        }
    }
    best
}
