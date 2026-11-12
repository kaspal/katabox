use std::collections::VecDeque;

pub fn window_swings(readings: Vec<i64>, k: i64) -> Vec<i64> {
    let k = k as usize;
    let mut maxq: VecDeque<usize> = VecDeque::new();
    let mut minq: VecDeque<usize> = VecDeque::new();
    let mut out = Vec::new();
    for (i, &r) in readings.iter().enumerate() {
        while maxq.back().is_some_and(|&j| readings[j] <= r) {
            maxq.pop_back();
        }
        maxq.push_back(i);
        while minq.back().is_some_and(|&j| readings[j] >= r) {
            minq.pop_back();
        }
        minq.push_back(i);
        if maxq[0] + k <= i {
            maxq.pop_front();
        }
        if minq[0] + k <= i {
            minq.pop_front();
        }
        if i + 1 >= k {
            out.push(readings[maxq[0]] - readings[minq[0]]);
        }
    }
    out
}
