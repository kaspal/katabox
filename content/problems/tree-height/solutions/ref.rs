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

pub fn tree_height(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    match root {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            1 + tree_height(n.left.clone()).max(tree_height(n.right.clone()))
        }
    }
}
