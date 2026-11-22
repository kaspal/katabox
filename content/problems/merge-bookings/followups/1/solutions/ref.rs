fn merge(bookings: Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    let mut v: Vec<(i64, i64)> = bookings.iter().map(|b| (b[0], b[1])).collect();
    v.sort_unstable();
    let mut out: Vec<(i64, i64)> = Vec::new();
    for (s, e) in v {
        match out.last_mut() {
            Some(last) if s <= last.1 => last.1 = last.1.max(e),
            _ => out.push((s, e)),
        }
    }
    out
}

pub fn both_busy(room_a: Vec<Vec<i64>>, room_b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (a, b) = (merge(room_a), merge(room_b));
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < a.len() && j < b.len() {
        let (s, e) = (a[i].0.max(b[j].0), a[i].1.min(b[j].1));
        if s <= e {
            out.push(vec![s, e]);
        }
        if a[i].1 < b[j].1 {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}
