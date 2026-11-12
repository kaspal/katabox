pub fn common_boss(boss: Vec<i64>, pairs: Vec<Vec<i64>>) -> Vec<i64> {
    // Correct but O(q · depth): mark everyone above a, then walk up from b.
    let n = boss.len();
    let mut mark = vec![usize::MAX; n];
    pairs
        .iter()
        .enumerate()
        .map(|(j, p)| {
            let mut v = p[0];
            while v >= 0 {
                mark[v as usize] = j;
                v = boss[v as usize];
            }
            let mut v = p[1];
            while mark[v as usize] != j {
                v = boss[v as usize];
            }
            v
        })
        .collect()
}
