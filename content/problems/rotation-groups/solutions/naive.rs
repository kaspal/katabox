pub fn rotation_groups(codes: Vec<String>) -> i64 {
    // Correct but O(n²·L): compare each code with every earlier code it might repeat.
    let mut count = 0;
    for i in 0..codes.len() {
        let a = &codes[i];
        let mut new = true;
        for b in &codes[..i] {
            if a.len() == b.len() && format!("{b}{b}").contains(a.as_str()) {
                new = false;
                break;
            }
        }
        if new {
            count += 1;
        }
    }
    count
}
