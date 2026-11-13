pub fn largest_poster(panels: Vec<i64>) -> i64 {
    let n = panels.len();
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    let mut best = 0;
    for i in 0..=n {
        let h = if i < n { panels[i] } else { 0 };
        while let Some(&top) = stack.last() {
            if panels[top] <= h {
                break;
            }
            stack.pop();
            let width = match stack.last() {
                Some(&left) => i - left - 1,
                None => i,
            };
            best = best.max(panels[top] * width as i64);
        }
        stack.push(i);
    }
    best
}
