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

fn gain(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i64) -> i64 {
    let Some(n) = node else { return 0 };
    let n = n.borrow();
    let a = gain(&n.left, best).max(0);
    let b = gain(&n.right, best).max(0);
    *best = (*best).max(n.val + a + b);
    n.val + a.max(b)
}

pub fn richest_trail(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    let mut best = i64::MIN;
    gain(&root, &mut best);
    best
}
