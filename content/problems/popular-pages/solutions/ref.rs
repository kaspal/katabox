use std::collections::{BTreeSet, HashMap};

pub fn popular_pages(capacity: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // page -> (value, count, last use); the eviction order is (count, last use, page).
    let mut cache: HashMap<i64, (i64, i64, usize)> = HashMap::new();
    let mut order: BTreeSet<(i64, usize, i64)> = BTreeSet::new();
    let mut out = Vec::new();
    for (now, (op, a)) in ops.iter().zip(&args).enumerate() {
        let page = a[0];
        if let Some(entry) = cache.get_mut(&page) {
            order.remove(&(entry.1, entry.2, page));
            entry.1 += 1;
            entry.2 = now;
            if op == "get" {
                out.push(entry.0);
            } else {
                entry.0 = a[1];
            }
            order.insert((entry.1, entry.2, page));
        } else if op == "get" {
            out.push(-1);
        } else {
            if cache.len() as i64 == capacity {
                let (_, _, old) = order.pop_first().unwrap();
                cache.remove(&old);
            }
            cache.insert(page, (a[1], 1, now));
            order.insert((1, now, page));
        }
    }
    out
}
