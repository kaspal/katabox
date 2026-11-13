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

pub fn split_train(train: Option<Box<ListNode>>, k: i64) -> Vec<Option<Box<ListNode>>> {
    let mut n = 0i64;
    let mut node = train.as_deref();
    while let Some(x) = node {
        n += 1;
        node = x.next.as_deref();
    }
    let (base, extra) = (n / k, n % k);
    let mut rest = train;
    let mut parts = Vec::new();
    for i in 0..k {
        let size = base + if i < extra { 1 } else { 0 };
        if size == 0 {
            parts.push(None);
            continue;
        }
        let mut head = rest;
        let mut last = head.as_mut().unwrap();
        for _ in 0..size - 1 {
            last = last.next.as_mut().unwrap();
        }
        rest = last.next.take();
        parts.push(head);
    }
    parts
}
