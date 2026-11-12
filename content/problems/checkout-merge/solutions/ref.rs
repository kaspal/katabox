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

pub fn merge_lines(lines: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut fronts = lines;
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = fronts
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.as_ref().map(|n| Reverse((n.val, i))))
        .collect();
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while let Some(Reverse((_, i))) = heap.pop() {
        let mut node = fronts[i].take().unwrap();
        fronts[i] = node.next.take();
        if let Some(next) = &fronts[i] {
            heap.push(Reverse((next.val, i)));
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}
