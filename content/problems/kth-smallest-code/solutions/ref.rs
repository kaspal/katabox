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

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i64) -> i64 {
    let mut stack = Vec::new();
    let mut node = root;
    let mut k = k;
    loop {
        while let Some(n) = node {
            node = n.borrow().left.clone();
            stack.push(n);
        }
        let top = stack.pop().unwrap();
        k -= 1;
        if k == 0 {
            return top.borrow().val;
        }
        node = top.borrow().right.clone();
    }
}
