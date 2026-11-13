use std::collections::HashMap;

fn free(pos: usize, s: i64, n: usize, total: i64, memo: &mut HashMap<(usize, i64), i64>) -> i64 {
    if s > total {
        return 0;
    }
    if pos == n {
        return i64::from(s == total);
    }
    if let Some(&v) = memo.get(&(pos, s)) {
        return v;
    }
    let count = (0..10).map(|x| free(pos + 1, s + x, n, total, memo)).sum();
    memo.insert((pos, s), count);
    count
}

pub fn digit_budget(limit: i64, total: i64) -> i64 {
    let digits: Vec<i64> = limit.to_string().bytes().map(|b| (b - b'0') as i64).collect();
    let n = digits.len();
    let mut memo = HashMap::new();
    let (mut count, mut s) = (0, 0);
    for (pos, &top) in digits.iter().enumerate() {
        for x in 0..top {
            count += free(pos + 1, s + x, n, total, &mut memo);
        }
        s += top;
    }
    count + i64::from(s == total)
}
