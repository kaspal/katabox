pub fn users_with_all(users: Vec<i64>, queries: Vec<i64>) -> Vec<i64> {
    // Correct but O(n·q): test every user for every question.
    queries
        .iter()
        .map(|&q| users.iter().filter(|&&u| u & q == q).count() as i64)
        .collect()
}
