fn go(p: &[i64], i: usize, j: usize) -> i64 {
    if j - i < 2 {
        return 0;
    }
    (p[j] - p[i]) + (i + 1..j).map(|k| go(p, i, k) + go(p, k, j)).min().unwrap()
}

pub fn cut_cost(length: i64, marks: Vec<i64>) -> i64 {
    // Correct but exponential: try every first cut recursively, remembering nothing.
    let mut p = vec![0];
    let mut sorted = marks;
    sorted.sort();
    p.extend(sorted);
    p.push(length);
    go(&p, 0, p.len() - 1)
}
