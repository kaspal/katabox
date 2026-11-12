use std::collections::HashMap;

pub fn holds(ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(ops²): a plain list per book; cancelling searches the whole line.
    let mut lines: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut reports = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let line = lines.entry(a[0]).or_default();
        match op.as_str() {
            "hold" => line.push(a[1]),
            "cancel" => {
                if let Some(k) = line.iter().position(|&m| m == a[1]) {
                    line.remove(k);
                }
            }
            _ => reports.push(if line.is_empty() { -1 } else { line.remove(0) }),
        }
    }
    reports
}
