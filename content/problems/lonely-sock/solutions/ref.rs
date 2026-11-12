pub fn lone_sock(ids: Vec<i64>) -> i64 {
    ids.iter().fold(0, |acc, x| acc ^ x)
}
