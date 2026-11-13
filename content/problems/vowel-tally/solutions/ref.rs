pub fn vowel_tally(text: String) -> i64 {
    text.chars()
        .filter(|c| "aeiou".contains(c.to_ascii_lowercase()))
        .count() as i64
}
