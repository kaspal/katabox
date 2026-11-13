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

fn best(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
    let Some(n) = node else { return 0 };
    let n = n.borrow();
    let mut lit = n.val;
    for child in [&n.left, &n.right].into_iter().flatten() {
        let c = child.borrow();
        lit += best(&c.left) + best(&c.right);
    }
    lit.max(best(&n.left) + best(&n.right))
}

pub fn brightest_street(root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
    // Correct but exponential: "lit here and best of the grandchildren" versus "best of
    // the children", recomputed every time.
    best(&root)
}
