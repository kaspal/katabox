use std::collections::HashMap;

pub fn blocked_walks(rows: i64, cols: i64, blocked: Vec<Vec<i64>>) -> i64 {
    // Correct but O(rows · cols): count walks crossing by crossing, one row at a time.
    const P: i64 = 1_000_003;
    let mut closed: HashMap<i64, Vec<usize>> = HashMap::new();
    for x in &blocked {
        closed.entry(x[0]).or_default().push(x[1] as usize);
    }
    let cols = cols as usize;
    let mut row = vec![0i64; cols + 1];
    let mut shut = vec![false; cols + 1];
    for r in 0..=rows {
        let list = closed.get(&r);
        if let Some(list) = list {
            for &c in list {
                shut[c] = true;
            }
        }
        for c in 0..=cols {
            if shut[c] {
                row[c] = 0;
            } else if r == 0 && c == 0 {
                row[c] = 1;
            } else if c > 0 {
                row[c] = (row[c] + row[c - 1]) % P;
            }
        }
        if let Some(list) = list {
            for &c in list {
                shut[c] = false;
            }
        }
    }
    row[cols]
}
