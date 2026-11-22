pub fn exact_baskets(weights: Vec<i64>, k: i64, target: i64) -> Vec<Vec<i64>> {
    fn go(items: &[i64], i: usize, k: usize, left: i64, pick: &mut Vec<i64>, out: &mut Vec<Vec<i64>>) {
        if pick.len() == k {
            if left == 0 {
                out.push(pick.clone());
            }
            return;
        }
        for j in i..items.len() {
            pick.push(items[j]);
            go(items, j + 1, k, left - items[j], pick, out);
            pick.pop();
        }
    }
    let mut items = weights;
    items.sort();
    let mut out = Vec::new();
    go(&items, 0, k as usize, target, &mut Vec::new(), &mut out);
    out
}
