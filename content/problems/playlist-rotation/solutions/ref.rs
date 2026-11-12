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
    let mut n = 0i64;
    let mut node = playlist.as_deref();
    while let Some(x) = node {
        n += 1;
        node = x.next.as_deref();
    }
    if n == 0 {
        return None;
    }
    let r = k % n;
    if r == 0 {
        return playlist;
    }
    // Detach the first n - r songs, then hang them after the last r.
    let mut head = playlist;
    let mut cut = head.as_mut().unwrap();
    for _ in 0..n - r - 1 {
        cut = cut.next.as_mut().unwrap();
    }
    let mut front = cut.next.take();
    let mut tail = front.as_mut().unwrap();
    while tail.next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = head;
    front
}
