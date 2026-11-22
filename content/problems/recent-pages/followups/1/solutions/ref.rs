use std::collections::{BTreeSet, HashMap};

pub fn favourite_pages(capacity: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // page -> (value, uses, last use); order: (uses, last use, page), the first is evicted.
    let mut pages: HashMap<i64, (i64, i64, i64)> = HashMap::new();
    let mut order: BTreeSet<(i64, i64, i64)> = BTreeSet::new();
    let mut out = Vec::new();
    for (tick, (op, a)) in ops.iter().zip(args.iter()).enumerate() {
        let tick = tick as i64;
        if op == "get" {
            match pages.get_mut(&a[0]) {
                Some(e) => {
                    order.remove(&(e.1, e.2, a[0]));
                    e.1 += 1;
                    e.2 = tick;
                    order.insert((e.1, e.2, a[0]));
                    out.push(e.0);
                }
                None => out.push(-1),
            }
            continue;
        }
        let (page, v) = (a[0], a[1]);
        if let Some(e) = pages.get_mut(&page) {
            order.remove(&(e.1, e.2, page));
            e.0 = v;
            e.1 += 1;
            e.2 = tick;
            order.insert((e.1, e.2, page));
            continue;
        }
        if pages.len() as i64 == capacity {
            if let Some(&first) = order.iter().next() {
                order.remove(&first);
                pages.remove(&first.2);
            }
        }
        pages.insert(page, (v, 1, tick));
        order.insert((1, tick, page));
    }
    out
}
