pub fn insert_slot(slots: Vec<Vec<i64>>, fresh: Vec<i64>) -> Vec<Vec<i64>> {
    let (mut s, mut e) = (fresh[0], fresh[1]);
    let mut out = Vec::with_capacity(slots.len() + 1);
    let mut i = 0;
    while i < slots.len() && slots[i][1] < s {
        out.push(slots[i].clone());
        i += 1;
    }
    while i < slots.len() && slots[i][0] <= e {
        s = s.min(slots[i][0]);
        e = e.max(slots[i][1]);
        i += 1;
    }
    out.push(vec![s, e]);
    out.extend_from_slice(&slots[i..]);
    out
}
