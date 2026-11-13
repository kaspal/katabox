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

pub fn window_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i64> {
    let mut seen = Vec::new();
    let mut level: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
    while let Some(last) = level.last() {
        seen.push(last.borrow().val);
        let mut next = Vec::new();
        for node in &level {
            let n = node.borrow();
            next.extend(n.left.clone());
            next.extend(n.right.clone());
        }
        level = next;
    }
    seen
}
