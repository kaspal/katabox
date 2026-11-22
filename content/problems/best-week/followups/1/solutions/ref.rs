use std::collections::VecDeque;

pub fn best_run(profits: Vec<i64>, k: i64) -> i64 {
    let n = profits.len();
    let k = k as usize;
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + profits[i];
    }
    let mut lows: VecDeque<usize> = VecDeque::new();
    let mut best = i64::MIN;
    for j in 1..=n {
        let i = j - 1;
        while lows.back().is_some_and(|&b| prefix[b] >= prefix[i]) {
            lows.pop_back();
        }
        lows.push_back(i);
        if lows.front().is_some_and(|&f| f + k < j) {
            lows.pop_front();
        }
        best = best.max(prefix[j] - prefix[lows[0]]);
    }
    best
}
