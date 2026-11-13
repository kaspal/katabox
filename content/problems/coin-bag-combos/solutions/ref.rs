fn pick(coins: &[i64], start: usize, remaining: i64, bag: &mut Vec<i64>, out: &mut Vec<Vec<i64>>) {
    if remaining == 0 {
        out.push(bag.clone());
        return;
    }
    for i in start..coins.len() {
        if coins[i] > remaining {
            break;
        }
        bag.push(coins[i]);
        pick(coins, i, remaining - coins[i], bag, out);
        bag.pop();
    }
}

pub fn coin_combos(coins: Vec<i64>, target: i64) -> Vec<Vec<i64>> {
    let mut coins = coins;
    coins.sort();
    let mut out = Vec::new();
    pick(&coins, 0, target, &mut Vec::new(), &mut out);
    out
}
