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
    let mut n = 0usize;
    let mut node = queue.as_deref();
    while let Some(x) = node {
        n += 1;
        node = x.next.as_deref();
    }
    if n < 3 {
        return queue;
    }
    // Cut after the first ceil(n / 2) nodes.
    let mut head = queue;
    let mut cut = head.as_mut().unwrap();
    for _ in 0..n.div_ceil(2) - 1 {
        cut = cut.next.as_mut().unwrap();
    }
    let mut back = cut.next.take();
    // Reverse the back half.
    let mut reversed: Option<Box<ListNode>> = None;
    while let Some(mut x) = back {
        back = x.next.take();
        x.next = reversed;
        reversed = Some(x);
    }
    // Interleave: front, back, front, back, ...
    let mut front = head;
    let mut out = Box::new(ListNode::new(0));
    let mut tail = &mut out;
    while let Some(mut f) = front {
        front = f.next.take();
        tail.next = Some(f);
        tail = tail.next.as_mut().unwrap();
        if let Some(mut b) = reversed {
            reversed = b.next.take();
            tail.next = Some(b);
            tail = tail.next.as_mut().unwrap();
        }
    }
    out.next
}
