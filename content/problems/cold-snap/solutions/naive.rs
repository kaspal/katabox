pub fn cold_snaps(temps: Vec<i64>, ops: Vec<String>, args: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·q): every operation walks its range.
    let mut t = temps;
    let mut out = Vec::new();
    for (op, a) in ops.iter().zip(&args) {
        let range = a[0] as usize..=a[1] as usize;
        if op == "shift" {
            for v in &mut t[range] {
                *v += a[2];
            }
        } else {
            out.push(*t[range].iter().min().unwrap());
        }
    }
    out
}
