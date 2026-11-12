pub fn painted_length(strokes: Vec<Vec<i64>>) -> i64 {
    let mut s: Vec<(i64, i64)> = strokes.iter().map(|v| (v[0], v[1])).collect();
    s.sort_unstable();
    let mut total = 0;
    let (mut run_start, mut run_end) = s[0];
    for &(start, end) in &s[1..] {
        if start <= run_end {
            run_end = run_end.max(end);
        } else {
            total += run_end - run_start;
            run_start = start;
            run_end = end;
        }
    }
    total + run_end - run_start
}
