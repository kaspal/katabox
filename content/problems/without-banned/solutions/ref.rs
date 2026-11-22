use std::collections::HashSet;

pub fn without_banned(items: Vec<i64>, banned: Vec<i64>) -> Vec<i64> {
    let bad: HashSet<i64> = banned.into_iter().collect();
    items.into_iter().filter(|x| !bad.contains(x)).collect()
}
