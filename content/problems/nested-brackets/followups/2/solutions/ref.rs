pub fn longest_balanced_part(text: String) -> i64 {
    let t = text.as_bytes();
    let mut stack: Vec<i64> = vec![-1];
    let mut best = 0;
    for (i, &c) in t.iter().enumerate() {
        let i = i as i64;
        let want = match c {
            b')' => b'(',
            b']' => b'[',
            b'}' => b'{',
            _ => {
                stack.push(i);
                continue;
            }
        };
        if stack.len() > 1 && t[*stack.last().unwrap() as usize] == want {
            stack.pop();
            best = best.max(i - stack.last().unwrap());
        } else {
            stack = vec![i];
        }
    }
    best
}
