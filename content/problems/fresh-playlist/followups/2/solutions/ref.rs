use std::collections::HashMap;

pub fn longest_rotation(plays: Vec<i64>, k: i64) -> i64 {
    let k = k as usize;
    let mut count: HashMap<i64, i64> = HashMap::new();
    let (mut best, mut start) = (0usize, 0usize);
    for (end, &song) in plays.iter().enumerate() {
        *count.entry(song).or_insert(0) += 1;
        while count.len() > k {
            let s = plays[start];
            let c = count.get_mut(&s).unwrap();
            *c -= 1;
            if *c == 0 {
                count.remove(&s);
            }
            start += 1;
        }
        best = best.max(end - start + 1);
    }
    best as i64
}
