pub fn best_haul(weights: Vec<i64>, prices: Vec<i64>, capacity: i64) -> f64 {
    let mut order: Vec<usize> = (0..weights.len()).collect();
    order.sort_by(|&i, &j| (prices[j] * weights[i]).cmp(&(prices[i] * weights[j])));
    let mut whole = 0i64;
    let mut room = capacity;
    for &i in &order {
        if weights[i] <= room {
            whole += prices[i];
            room -= weights[i];
        } else {
            return whole as f64 + (prices[i] * room) as f64 / weights[i] as f64;
        }
    }
    whole as f64
}
