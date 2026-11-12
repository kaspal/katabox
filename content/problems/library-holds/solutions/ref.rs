use std::collections::{HashMap, VecDeque};

pub fn holds(ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let mut line: HashMap<i64, VecDeque<(i64, u32)>> = HashMap::new();
    let mut waiting: HashMap<(i64, i64), u32> = HashMap::new();
    let mut tickets = 0u32;
    let mut reports = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let book = a[0];
        match op.as_str() {
            "hold" => {
                tickets += 1;
                waiting.insert((book, a[1]), tickets);
                line.entry(book).or_default().push_back((a[1], tickets));
            }
            "cancel" => {
                waiting.remove(&(book, a[1]));
            }
            _ => {
                let mut got = -1;
                if let Some(q) = line.get_mut(&book) {
                    while let Some((m, t)) = q.pop_front() {
                        if waiting.get(&(book, m)) == Some(&t) {
                            waiting.remove(&(book, m));
                            got = m;
                            break;
                        }
                    }
                }
                reports.push(got);
            }
        }
    }
    reports
}
