pub fn medal_table(countries: Vec<String>, medals: Vec<Vec<i64>>) -> Vec<String> {
    let mut order: Vec<usize> = (0..countries.len()).collect();
    // More medals first (gold, then silver, then bronze), then names alphabetically.
    order.sort_by(|&a, &b| medals[b].cmp(&medals[a]).then_with(|| countries[a].cmp(&countries[b])));
    order.into_iter().map(|i| countries[i].clone()).collect()
}
