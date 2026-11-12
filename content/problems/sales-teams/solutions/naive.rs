pub fn team_totals(boss: Vec<i64>, sales: Vec<i64>) -> Vec<i64> {
    // Correct but O(n · depth): add each employee's sales to every boss above them.
    let mut total = sales.clone();
    for e in 0..boss.len() {
        let mut b = boss[e];
        while b >= 0 {
            total[b as usize] += sales[e];
            b = boss[b as usize];
        }
    }
    total
}
