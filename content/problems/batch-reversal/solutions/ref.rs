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

pub fn reverse_batches(line: Option<Box<ListNode>>, k: i64) -> Option<Box<ListNode>> {
    let k = k as usize;
    let mut rest = line;
    let mut out = Box::new(ListNode::new(0));
    let mut tail = &mut out;
    loop {
        // Is there a full batch ahead?
        let mut count = 0;
        let mut probe = rest.as_deref();
        while let Some(x) = probe {
            if count == k {
                break;
            }
            count += 1;
            probe = x.next.as_deref();
        }
        if count < k {
            tail.next = rest;
            return out.next;
        }
        // Reverse k nodes off the front of rest.
        let mut reversed: Option<Box<ListNode>> = None;
        for _ in 0..k {
            let mut x = rest.unwrap();
            rest = x.next.take();
            x.next = reversed;
            reversed = Some(x);
        }
        tail.next = reversed;
        for _ in 0..k {
            tail = tail.next.as_mut().unwrap();
        }
    }
}
