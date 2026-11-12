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
    // Correct but O(N·k): look at every line's front for each customer.
    let mut fronts = lines;
    let mut out: Vec<i64> = Vec::new();
    loop {
        let mut best: Option<usize> = None;
        for (i, node) in fronts.iter().enumerate() {
            if let Some(n) = node {
                if best.is_none_or(|b| n.val < fronts[b].as_ref().unwrap().val) {
                    best = Some(i);
                }
            }
        }
        let Some(b) = best else { break };
        let mut node = fronts[b].take().unwrap();
        fronts[b] = node.next.take();
        out.push(node.val);
    }
    let mut head = None;
    for v in out.into_iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}
