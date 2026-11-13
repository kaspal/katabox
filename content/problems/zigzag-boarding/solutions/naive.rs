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

pub fn zigzag_boarding(queue: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Correct but O(n²): after each placed passenger, walk to the end for the last one.
    let mut head = queue;
    let mut node = head.as_mut();
    while let Some(cur) = node {
        let has_three = cur.next.as_ref().is_some_and(|x| x.next.is_some());
        if !has_three {
            break;
        }
        let mut before = cur.next.as_mut().unwrap();
        while before.next.as_ref().unwrap().next.is_some() {
            before = before.next.as_mut().unwrap();
        }
        let mut last = before.next.take().unwrap();
        last.next = cur.next.take();
        cur.next = Some(last);
        node = cur.next.as_mut().unwrap().next.as_mut();
    }
    head
}
