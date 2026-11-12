use std::collections::HashMap;

pub fn shop(ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let mut shelf: HashMap<i64, i64> = HashMap::new();
    let mut reports = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let have = shelf.entry(a[0]).or_insert(0);
        match op.as_str() {
            "stock" => *have += a[1],
            "sell" => {
                let sold = a[1].min(*have);
                *have -= sold;
                reports.push(sold);
            }
            _ => reports.push(*have),
        }
    }
    reports
}
