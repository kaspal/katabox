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

pub fn rotate_playlist(playlist: Option<Box<ListNode>>, k: i64) -> Option<Box<ListNode>> {
    // Correct but O(n·(k mod n)): one walk to the end per press that matters.
    let mut head = playlist;
    let mut n = 0i64;
    let mut node = head.as_deref();
    while let Some(x) = node {
        n += 1;
        node = x.next.as_deref();
    }
    if n < 2 {
        return head;
    }
    for _ in 0..k % n {
        let mut before = head.as_mut().unwrap();
        while before.next.as_ref().unwrap().next.is_some() {
            before = before.next.as_mut().unwrap();
        }
        let mut last = before.next.take().unwrap();
        last.next = head;
        head = Some(last);
    }
    head
}
