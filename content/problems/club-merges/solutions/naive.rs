pub fn clubs_left(n: i64, merges: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(n·m): a club label per player, and every merge relabels by
    // scanning all players.
    let n = n as usize;
    let mut label: Vec<usize> = (0..n).collect();
    let mut clubs = n as i64;
    merges
        .iter()
        .map(|m| {
            let (la, lb) = (label[m[0] as usize], label[m[1] as usize]);
            if la != lb {
                for l in label.iter_mut() {
                    if *l == lb {
                        *l = la;
                    }
                }
                clubs -= 1;
            }
            clubs
        })
        .collect()
}
