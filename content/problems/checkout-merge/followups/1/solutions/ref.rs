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

pub fn merge_distinct(lines: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut tickets = Vec::new();
    for line in &lines {
        let mut node = line.as_deref();
        while let Some(n) = node {
            tickets.push(n.val);
            node = n.next.as_deref();
        }
    }
    tickets.sort_unstable();
    tickets.dedup();
    let mut head: Option<Box<ListNode>> = None;
    for &t in tickets.iter().rev() {
        head = Some(Box::new(ListNode { val: t, next: head }));
    }
    head
}
