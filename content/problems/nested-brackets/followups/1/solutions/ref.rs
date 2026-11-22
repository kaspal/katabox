pub fn fewest_insertions(text: String) -> i64 {
    let (mut open, mut added) = (0i64, 0i64);
    for b in text.bytes() {
        if b == b'(' {
            open += 1;
        } else if b == b')' {
            if open > 0 {
                open -= 1;
            } else {
                added += 1;
            }
        }
    }
    added + open
}
