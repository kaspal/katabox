pub fn most_snacks(prices: Vec<i64>, budget: i64) -> i64 {
    // Correct but O(n²): scan for the cheapest snack still on the shelf, again and again.
    let mut left = prices;
    let mut budget = budget;
    let mut count = 0;
    while !left.is_empty() {
        let mut best = 0;
        for i in 1..left.len() {
            if left[i] < left[best] {
                best = i;
            }
        }
        if left[best] > budget {
            break;
        }
        budget -= left[best];
        left.swap_remove(best);
        count += 1;
    }
    count
}
