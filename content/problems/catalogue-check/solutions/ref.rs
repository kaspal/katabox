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

fn ok(node: &Option<Rc<RefCell<TreeNode>>>, low: Option<i64>, high: Option<i64>) -> bool {
    let Some(n) = node else { return true };
    let n = n.borrow();
    if low.is_some_and(|l| n.val <= l) || high.is_some_and(|h| n.val >= h) {
        return false;
    }
    ok(&n.left, low, Some(n.val)) && ok(&n.right, Some(n.val), high)
}

pub fn is_search_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    ok(&root, None, None)
}
