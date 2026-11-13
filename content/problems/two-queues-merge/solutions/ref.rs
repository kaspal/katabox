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

pub fn merge_queues(first: Option<Box<ListNode>>, second: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    let (mut a, mut b) = (first, second);
    loop {
        match (a.take(), b.take()) {
            (Some(mut x), Some(mut y)) => {
                if x.val <= y.val {
                    a = x.next.take();
                    b = Some(y);
                    tail.next = Some(x);
                } else {
                    b = y.next.take();
                    a = Some(x);
                    tail.next = Some(y);
                }
                tail = tail.next.as_mut().unwrap();
            }
            (x, y) => {
                tail.next = x.or(y);
                break;
            }
        }
    }
    head.next
}
