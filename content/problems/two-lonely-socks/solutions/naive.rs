pub fn two_lonely(ids: Vec<i64>) -> Vec<i64> {
    // Correct but O(n²): look for each sock's partner elsewhere in the list.
    let n = ids.len();
    let mut out = Vec::new();
    for i in 0..n {
        let mut paired = false;
        for j in 0..n {
            if j != i && ids[j] == ids[i] {
                paired = true;
                break;
            }
        }
        if !paired {
            out.push(ids[i]);
        }
    }
    out.sort_unstable();
    out
}
