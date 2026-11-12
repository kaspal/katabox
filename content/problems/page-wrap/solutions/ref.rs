pub fn wrap_lines(words: Vec<String>, width: i64) -> Vec<String> {
    let width = width as usize;
    let mut lines = Vec::new();
    let mut current = words[0].clone();
    for w in &words[1..] {
        if current.len() + 1 + w.len() <= width {
            current.push(' ');
            current.push_str(w);
        } else {
            lines.push(std::mem::replace(&mut current, w.clone()));
        }
    }
    lines.push(current);
    lines
}
