pub fn team_totals(boss: Vec<i64>, sales: Vec<i64>) -> Vec<i64> {
    let n = boss.len();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut owner = 0;
    for (e, &b) in boss.iter().enumerate() {
        if b < 0 {
            owner = e;
        } else {
            children[b as usize].push(e);
        }
    }
    let mut order = vec![owner];
    let mut k = 0;
    while k < order.len() {
        let e = order[k];
        order.extend_from_slice(&children[e]);
        k += 1;
    }
    let mut total = sales;
    for &e in order.iter().rev() {
        if boss[e] >= 0 {
            total[boss[e] as usize] += total[e];
        }
    }
    total
}
