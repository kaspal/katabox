use std::collections::HashMap;

pub fn fresh_stretch(plays: Vec<i64>) -> i64 {
    let mut last: HashMap<i64, usize> = HashMap::with_capacity(plays.len());
    let (mut start, mut best) = (0, 0);
    for (end, &x) in plays.iter().enumerate() {
        if let Some(&seen) = last.get(&x) {
            if seen >= start {
                start = seen + 1;
            }
        }
        last.insert(x, end);
        best = best.max(end - start + 1);
    }
    best as i64
}
