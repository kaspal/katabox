pub fn first_at_least(sorted: Vec<i64>, x: i64) -> i64 {
    sorted.partition_point(|&v| v < x) as i64
}
