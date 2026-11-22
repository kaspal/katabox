pub fn longest_run(values: Vec<i64>) -> i64 {
    let (mut best, mut run) = (1, 1);
    for i in 1..values.len() {
        run = if values[i] == values[i - 1] { run + 1 } else { 1 };
        best = best.max(run);
    }
    best
}
