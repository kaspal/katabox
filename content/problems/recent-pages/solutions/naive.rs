pub fn recent_pages(capacity: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(capacity) per operation: a list of (page, value), oldest first.
    let mut cache: Vec<(i64, i64)> = Vec::new();
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let page = a[0];
        let at = cache.iter().position(|e| e.0 == page);
        if op == "get" {
            match at {
                Some(i) => {
                    let entry = cache.remove(i);
                    cache.push(entry);
                    out.push(entry.1);
                }
                None => out.push(-1),
            }
        } else {
            if let Some(i) = at {
                cache.remove(i);
            }
            cache.push((page, a[1]));
            if cache.len() as i64 > capacity {
                cache.remove(0);
            }
        }
    }
    out
}
