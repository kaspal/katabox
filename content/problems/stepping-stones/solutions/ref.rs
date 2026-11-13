pub fn can_cross(jumps: Vec<i64>) -> bool {
    let mut reach = 0i64;
    for (i, &j) in jumps.iter().enumerate() {
        let i = i as i64;
        if i > reach {
            return false;
        }
        reach = reach.max(i + j);
    }
    true
}
