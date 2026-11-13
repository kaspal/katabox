pub fn merge_bookings(bookings: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut sorted = bookings;
    sorted.sort();
    let mut out: Vec<Vec<i64>> = Vec::new();
    for b in sorted {
        match out.last_mut() {
            Some(last) if b[0] <= last[1] => last[1] = last[1].max(b[1]),
            _ => out.push(b),
        }
    }
    out
}
