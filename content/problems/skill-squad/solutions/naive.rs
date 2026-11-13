pub fn smallest_squad(skills: i64, people: Vec<i64>) -> i64 {
    // Correct but exponential: try every team, smallest first.
    let full = (1i64 << skills) - 1;
    if people.iter().fold(0, |a, &p| a | p) != full {
        return -1;
    }
    fn search(people: &[i64], start: usize, left: usize, mask: i64, full: i64) -> bool {
        if left == 0 {
            return mask == full;
        }
        (start..people.len()).any(|i| search(people, i + 1, left - 1, mask | people[i], full))
    }
    for k in 1..=people.len() {
        if search(&people, 0, k, 0, full) {
            return k as i64;
        }
    }
    -1
}
