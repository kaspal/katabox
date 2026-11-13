pub fn popular_pages(capacity: i64, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(capacity) per operation: a plain list of (page, value, count, last use).
    let mut cache: Vec<(i64, i64, i64, usize)> = Vec::new();
    let mut out = Vec::new();
    for (now, (op, a)) in ops.iter().zip(&args).enumerate() {
        let page = a[0];
        if let Some(e) = cache.iter_mut().find(|e| e.0 == page) {
            e.2 += 1;
            e.3 = now;
            if op == "get" {
                out.push(e.1);
            } else {
                e.1 = a[1];
            }
        } else if op == "get" {
            out.push(-1);
        } else {
            if cache.len() as i64 == capacity {
                let victim = (0..cache.len()).min_by_key(|&i| (cache[i].2, cache[i].3)).unwrap();
                cache.swap_remove(victim);
            }
            cache.push((page, a[1], 1, now));
        }
    }
    out
}
