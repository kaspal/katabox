use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Provided by KataBox: the judge builds the input lists with this struct.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i64,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i64) -> Self {
        ListNode { next: None, val }
    }
}

pub fn fair_sample(lines: Vec<Option<Box<ListNode>>>) -> Vec<i64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    // Each line becomes a vector of its tickets; the heap holds (ticket, line, position).
    let tickets: Vec<Vec<i64>> = lines
        .iter()
        .map(|head| {
            let mut out = Vec::new();
            let mut node = head.as_ref();
            while let Some(n) = node {
                out.push(n.val);
                node = n.next.as_ref();
            }
            out
        })
        .collect();
    let mut heap = BinaryHeap::new();
    let mut largest = 0i64;
    for (i, t) in tickets.iter().enumerate() {
        heap.push(Reverse((t[0], i, 0usize)));
        largest = largest.max(t[0]);
    }
    let mut best: Option<(i64, i64)> = None;
    loop {
        let Reverse((low, i, pos)) = heap.pop().unwrap();
        if best.map_or(true, |(lo, hi)| largest - low < hi - lo) {
            best = Some((low, largest));
        }
        if pos + 1 == tickets[i].len() {
            let (lo, hi) = best.unwrap();
            return vec![lo, hi];
        }
        let next = tickets[i][pos + 1];
        largest = largest.max(next);
        heap.push(Reverse((next, i, pos + 1)));
    }
}
