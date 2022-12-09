/* https://leetcode.com/problems/same-tree/description/ */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

fn walk(p: &mut Option<Rc<RefCell<TreeNode>>>, q: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }

    if p.is_none() || q.is_none() {
        return false;
    }

    if let Some(np) = p {
        if let Some(nq) = q {
            let mut np = np.borrow_mut();
            let mut nq = nq.borrow_mut();

            return walk(&mut np.left, &mut nq.left)
                && walk(&mut np.right, &mut nq.right)
                && np.val == nq.val;
        }
    }

    return false;
}

pub fn is_same_tree(
    mut p: Option<Rc<RefCell<TreeNode>>>,
    mut q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    walk(&mut p, &mut q)
}

fn main() {}
