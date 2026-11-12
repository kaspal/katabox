pub fn is_balanced(text: String) -> bool {
    // Correct, but each pass removes only the innermost pairs: O(n²) on deep nesting.
    let mut s: String = text.chars().filter(|c| "()[]{}".contains(*c)).collect();
    loop {
        let t = s
            .replacen("()", "", 1)
            .replacen("[]", "", 1)
            .replacen("{}", "", 1);
        if t == s {
            return s.is_empty();
        }
        s = t;
    }
}
