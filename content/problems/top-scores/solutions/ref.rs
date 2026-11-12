use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn top_scores(scores: Vec<i64>, k: i64) -> Vec<i64> {
    let k = k as usize;
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::with_capacity(k + 1);
    for s in scores {
        if heap.len() < k {
            heap.push(Reverse(s));
        } else if let Some(&Reverse(low)) = heap.peek() {
            if s > low {
                heap.pop();
                heap.push(Reverse(s));
            }
        }
    }
    let mut out: Vec<i64> = heap.into_iter().map(|Reverse(s)| s).collect();
    out.sort_unstable_by(|a, b| b.cmp(a));
    out
}
