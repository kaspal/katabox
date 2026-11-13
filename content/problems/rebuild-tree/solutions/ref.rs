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

use std::collections::HashMap;

fn build(pre: &[i64], pos: &HashMap<i64, usize>, ps: usize, ins: usize, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if len == 0 {
        return None;
    }
    let mut root = TreeNode::new(pre[ps]);
    let left = pos[&pre[ps]] - ins;
    root.left = build(pre, pos, ps + 1, ins, left);
    root.right = build(pre, pos, ps + 1 + left, ins + left + 1, len - left - 1);
    Some(Rc::new(RefCell::new(root)))
}

pub fn rebuild_tree(preorder: Vec<i64>, inorder: Vec<i64>) -> Option<Rc<RefCell<TreeNode>>> {
    let pos: HashMap<i64, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    build(&preorder, &pos, 0, 0, preorder.len())
}
