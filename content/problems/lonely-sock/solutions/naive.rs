pub fn lone_sock(ids: Vec<i64>) -> i64 {
    // Correct but O(n²): look for each sock's partner elsewhere in the list.
    let n = ids.len();
    for i in 0..n {
        let mut paired = false;
        for j in 0..n {
            if j != i && ids[j] == ids[i] {
                paired = true;
                break;
            }
        }
        if !paired {
            return ids[i];
        }
    }
    -1
}
