use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kth_after_each(scores: Vec<i64>, k: i64) -> Vec<i64> {
    let k = k as usize;
    let mut top: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut out = Vec::with_capacity(scores.len());
    for s in scores {
        if top.len() < k {
            top.push(Reverse(s));
        } else if let Some(&Reverse(low)) = top.peek() {
            if s > low {
                top.pop();
                top.push(Reverse(s));
            }
        }
        out.push(if top.len() == k { top.peek().map_or(-1, |r| r.0) } else { -1 });
    }
    out
}
