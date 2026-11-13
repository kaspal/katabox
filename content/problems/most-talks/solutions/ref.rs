pub fn most_talks(talks: Vec<Vec<i64>>) -> i64 {
    let mut sorted = talks;
    sorted.sort_by_key(|t| t[1]);
    let (mut count, mut free) = (0, -1);
    for t in &sorted {
        if t[0] >= free {
            count += 1;
            free = t[1];
        }
    }
    count
}
