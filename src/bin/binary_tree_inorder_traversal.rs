/* https://leetcode.com/problems/binary-tree-inorder-traversal/description/ */

use std::cell::RefCell;
use std::rc::Rc;

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

pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];

    fn walk(node: &mut Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            walk(&mut n.left, ans);
            ans.push(n.val);
            walk(&mut n.right, ans);
        }
    }

    walk(&mut root, &mut answer);

    answer
}

fn main() {}
