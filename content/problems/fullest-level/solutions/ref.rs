use std::cell::RefCell;
use std::rc::Rc;

// Provided by KataBox: the judge builds the input trees with this struct.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i64,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i64) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn fullest_level(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    let Some(root) = root else { return 0 };
    let (mut best, mut best_level, mut level) = (-1i64, 0i64, 0i64);
    let mut queue = vec![root];
    while !queue.is_empty() {
        level += 1;
        let mut total = 0;
        let mut next = Vec::new();
        for node in &queue {
            let n = node.borrow();
            total += n.val;
            next.extend(n.left.clone());
            next.extend(n.right.clone());
        }
        if total > best {
            best = total;
            best_level = level;
        }
        queue = next;
    }
    best_level
}
