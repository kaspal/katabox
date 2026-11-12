use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn step_order(n: i64, before: Vec<Vec<i64>>) -> Vec<i64> {
    let n = n as usize;
    let mut waiting = vec![0usize; n];
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in &before {
        let (a, b) = (e[0] as usize, e[1] as usize);
        out[a].push(b);
        waiting[b] += 1;
    }
    let mut heap: BinaryHeap<Reverse<usize>> = (0..n).filter(|&s| waiting[s] == 0).map(Reverse).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse(s)) = heap.pop() {
        order.push(s as i64);
        for &t in &out[s] {
            waiting[t] -= 1;
            if waiting[t] == 0 {
                heap.push(Reverse(t));
            }
        }
    }
    if order.len() == n { order } else { Vec::new() }
}
