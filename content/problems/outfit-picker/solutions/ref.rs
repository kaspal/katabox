pub fn outfits(prices: Vec<Vec<i64>>, budget: i64) -> i64 {
    let mut groups = prices;
    for g in &mut groups {
        g.sort_unstable();
    }
    let mut cheapest = vec![0i64; groups.len() + 1];
    for c in (0..groups.len()).rev() {
        cheapest[c] = cheapest[c + 1] + groups[c][0];
    }
    fn choose(c: usize, spent: i64, groups: &[Vec<i64>], cheapest: &[i64], budget: i64) -> i64 {
        if c == groups.len() {
            return 1;
        }
        let mut total = 0;
        for &price in &groups[c] {
            if spent + price + cheapest[c + 1] > budget {
                break;
            }
            total += choose(c + 1, spent + price, groups, cheapest, budget);
        }
        total
    }
    choose(0, 0, &groups, &cheapest, budget)
}
