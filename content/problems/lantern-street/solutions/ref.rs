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

fn best(node: &Option<Rc<RefCell<TreeNode>>>) -> (i64, i64) {
    let Some(n) = node else { return (0, 0) };
    let n = n.borrow();
    let (l_lit, l_dark) = best(&n.left);
    let (r_lit, r_dark) = best(&n.right);
    (n.val + l_dark + r_dark, l_lit.max(l_dark) + r_lit.max(r_dark))
}

pub fn brightest_street(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    let (lit, dark) = best(&root);
    lit.max(dark)
}
