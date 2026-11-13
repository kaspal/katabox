use std::collections::{BTreeMap, HashMap};

pub fn recent_pages(capacity: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // page -> (value, last use); last use -> page, oldest first.
    let mut cache: HashMap<i64, (i64, usize)> = HashMap::new();
    let mut order: BTreeMap<usize, i64> = BTreeMap::new();
    let mut out = Vec::new();
    for (now, (op, a)) in ops.iter().zip(&args).enumerate() {
        let page = a[0];
        if op == "get" {
            match cache.get_mut(&page) {
                Some(entry) => {
                    order.remove(&entry.1);
                    entry.1 = now;
                    order.insert(now, page);
                    out.push(entry.0);
                }
                None => out.push(-1),
            }
        } else {
            if let Some(old) = cache.insert(page, (a[1], now)) {
                order.remove(&old.1);
            }
            order.insert(now, page);
            if cache.len() as i64 > capacity {
                let (_, oldest) = order.pop_first().unwrap();
                cache.remove(&oldest);
            }
        }
    }
    out
}
