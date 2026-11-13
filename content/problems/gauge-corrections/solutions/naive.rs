pub fn gauge_readings(readings: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·q): every "add" walks its range.
    let mut levels = readings;
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        if op == "add" {
            for v in &mut levels[a[0] as usize..=a[1] as usize] {
                *v += a[2];
            }
        } else {
            out.push(levels[a[0] as usize]);
        }
    }
    out
}
