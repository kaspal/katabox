pub fn generations(parent: Vec<i64>) -> i64 {
    let n = parent.len();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut founder = 0;
    for (i, &p) in parent.iter().enumerate() {
        if p < 0 {
            founder = i;
        } else {
            children[p as usize].push(i);
        }
    }
    let mut level = vec![founder];
    let mut count = 0;
    while !level.is_empty() {
        count += 1;
        level = level.iter().flat_map(|&p| children[p].iter().copied()).collect();
    }
    count
}
