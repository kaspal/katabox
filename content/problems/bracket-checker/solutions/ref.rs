pub fn brackets_ok(code: String) -> bool {
    let mut stack = Vec::new();
    for c in code.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            _ => if stack.pop() != Some('{') { return false; },
        }
    }
    stack.is_empty()
}
