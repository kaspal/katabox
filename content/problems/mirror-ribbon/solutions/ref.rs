pub fn is_mirror(text: String) -> bool {
    let b = text.as_bytes();
    if b.is_empty() {
        return true;
    }
    let (mut i, mut j) = (0usize, b.len() - 1);
    while i < j {
        if !b[i].is_ascii_alphanumeric() {
            i += 1;
        } else if !b[j].is_ascii_alphanumeric() {
            j -= 1;
        } else if b[i].to_ascii_lowercase() != b[j].to_ascii_lowercase() {
            return false;
        } else {
            i += 1;
            j -= 1;
        }
    }
    true
}
