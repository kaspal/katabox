pub fn first_matches(words: Vec<String>, prefixes: Vec<String>) -> Vec<String> {
    // Correct but O(n·q·L): scan every word for every prefix.
    prefixes
        .iter()
        .map(|p| {
            words
                .iter()
                .filter(|w| w.starts_with(p.as_str()))
                .min()
                .cloned()
                .unwrap_or_default()
        })
        .collect()
}
