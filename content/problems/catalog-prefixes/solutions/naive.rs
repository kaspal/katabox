pub fn prefix_counts(titles: Vec<String>, typed: Vec<String>) -> Vec<i64> {
    // Correct but O(N·Q·L): check every title for every search.
    typed
        .iter()
        .map(|p| titles.iter().filter(|t| t.starts_with(p.as_str())).count() as i64)
        .collect()
}
