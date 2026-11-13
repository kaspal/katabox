use std::collections::{HashMap, VecDeque};

pub fn throttle(limit: i64, window: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    let mut logs: HashMap<i64, VecDeque<i64>> = HashMap::new();
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let (user, time) = (a[0], a[1]);
        let log = logs.entry(user).or_default();
        while log.front().is_some_and(|&t| t <= time - window) {
            log.pop_front();
        }
        if op == "count" {
            out.push(log.len() as i64);
        } else if (log.len() as i64) < limit {
            log.push_back(time);
            out.push(1);
        } else {
            out.push(0);
        }
    }
    out
}
