use std::collections::HashMap;

pub fn notebook(ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let mut values: HashMap<i64, i64> = HashMap::new();
    let mut undo: Vec<(i64, Option<i64>, i64)> = Vec::new();
    let mut redo: Vec<(i64, Option<i64>, i64)> = Vec::new();
    let mut reports = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        match op.as_str() {
            "write" => {
                undo.push((a[0], values.get(&a[0]).copied(), a[1]));
                values.insert(a[0], a[1]);
                redo.clear();
            }
            "read" => reports.push(values.get(&a[0]).copied().unwrap_or(-1)),
            "undo" => {
                if let Some(c) = undo.pop() {
                    match c.1 {
                        Some(old) => values.insert(c.0, old),
                        None => values.remove(&c.0),
                    };
                    redo.push(c);
                }
            }
            _ => {
                if let Some(c) = redo.pop() {
                    values.insert(c.0, c.2);
                    undo.push(c);
                }
            }
        }
    }
    reports
}
