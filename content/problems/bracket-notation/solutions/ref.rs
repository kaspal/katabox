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

fn write(node: &Rc<RefCell<TreeNode>>, out: &mut String) {
    let n = node.borrow();
    out.push_str(&n.val.to_string());
    if n.left.is_some() || n.right.is_some() {
        out.push('(');
        if let Some(l) = &n.left {
            write(l, out);
        }
        out.push(')');
    }
    if let Some(r) = &n.right {
        out.push('(');
        write(r, out);
        out.push(')');
    }
}

pub fn bracket_notation(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut out = String::new();
    if let Some(r) = &root {
        write(r, &mut out);
    }
    out
}
