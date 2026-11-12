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

pub fn trim_hedge(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn trim(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i64) {
        let Some(n) = node else { return (None, 0) };
        let (left_in, right_in) = {
            let mut b = n.borrow_mut();
            (b.left.take(), b.right.take())
        };
        let (left, a) = trim(left_in);
        let (right, c) = trim(right_in);
        let total = n.borrow().val + a + c;
        if total < 0 {
            return (None, 0);
        }
        {
            let mut b = n.borrow_mut();
            b.left = left;
            b.right = right;
        }
        (Some(n), total)
    }
    trim(root).0
}
