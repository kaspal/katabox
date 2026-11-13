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

fn depth(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i64) -> i64 {
    let Some(n) = node else { return 0 };
    let n = n.borrow();
    let (a, b) = (depth(&n.left, best), depth(&n.right, best));
    *best = (*best).max(a + b);
    1 + a.max(b)
}

pub fn longest_walk(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    let mut best = 0;
    depth(&root, &mut best);
    best
}
