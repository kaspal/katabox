pub fn biggest_number(cards: Vec<i64>) -> String {
    let mut texts: Vec<String> = cards.iter().map(|c| c.to_string()).collect();
    texts.sort_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
    let joined = texts.concat();
    if joined.starts_with('0') {
        "0".to_string()
    } else {
        joined
    }
}
