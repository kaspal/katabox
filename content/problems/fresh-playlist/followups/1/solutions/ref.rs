use std::collections::HashMap;

pub fn fresh_with_encores(plays: Vec<i64>) -> i64 {
    let mut count: HashMap<i64, i64> = HashMap::new();
    let (mut best, mut start) = (0usize, 0usize);
    for (end, &song) in plays.iter().enumerate() {
        *count.entry(song).or_insert(0) += 1;
        while count[&song] > 2 {
            *count.get_mut(&plays[start]).unwrap() -= 1;
            start += 1;
        }
        best = best.max(end - start + 1);
    }
    best as i64
}
