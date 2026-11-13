pub fn title_case(text: String) -> String {
    let mut out = String::with_capacity(text.len());
    let mut at_start = true;
    for c in text.chars() {
        if c == ' ' {
            out.push(c);
            at_start = true;
        } else {
            out.push(if at_start { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() });
            at_start = false;
        }
    }
    out
}
