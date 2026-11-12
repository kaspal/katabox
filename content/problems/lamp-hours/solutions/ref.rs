pub fn lit_time(lamps: Vec<Vec<i64>>) -> i64 {
    let mut ordered: Vec<(i64, i64)> = lamps.iter().map(|l| (l[0], l[1])).collect();
    ordered.sort_unstable();
    let (mut s, mut e) = ordered[0];
    let mut total = 0;
    for &(start, end) in &ordered[1..] {
        if start <= e {
            e = e.max(end);
        } else {
            total += e - s;
            s = start;
            e = end;
        }
    }
    total + e - s
}
