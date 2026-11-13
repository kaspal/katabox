pub fn menu_combos(prices: Vec<i64>, budget: i64) -> Vec<Vec<i64>> {
    let n = prices.len();
    let mut out = Vec::new();
    for mask in 1u32..(1 << n) {
        let dishes: Vec<i64> = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| i as i64).collect();
        if dishes.iter().map(|&i| prices[i as usize]).sum::<i64>() == budget {
            out.push(dishes);
        }
    }
    out.sort();
    out
}
