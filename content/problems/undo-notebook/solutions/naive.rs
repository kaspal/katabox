pub fn notebook(ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(ops²): a read searches the writes in effect from the newest.
    let mut done: Vec<(i64, i64)> = Vec::new();
    let mut undone: Vec<(i64, i64)> = Vec::new();
    let mut reports = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        match op.as_str() {
            "write" => {
                done.push((a[0], a[1]));
                undone.clear();
            }
            "read" => reports.push(done.iter().rev().find(|w| w.0 == a[0]).map_or(-1, |w| w.1)),
            "undo" => {
                if let Some(w) = done.pop() {
                    undone.push(w);
                }
            }
            _ => {
                if let Some(w) = undone.pop() {
                    done.push(w);
                }
            }
        }
    }
    reports
}
