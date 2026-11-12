pub fn finish_order(names: Vec<String>, times: Vec<i64>) -> Vec<String> {
    let mut order: Vec<usize> = (0..names.len()).collect();
    order.sort_by(|&a, &b| times[a].cmp(&times[b]).then_with(|| names[a].cmp(&names[b])));
    order.into_iter().map(|i| names[i].clone()).collect()
}
