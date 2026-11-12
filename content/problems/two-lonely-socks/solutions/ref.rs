pub fn two_lonely(ids: Vec<i64>) -> Vec<i64> {
    let t = ids.iter().fold(0, |acc, &x| acc ^ x);
    let bit = t & -t;
    let a = ids.iter().filter(|&&x| x & bit != 0).fold(0, |acc, &x| acc ^ x);
    let b = t ^ a;
    vec![a.min(b), a.max(b)]
}
