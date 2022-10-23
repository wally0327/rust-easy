use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution {}

impl Solution {
    pub fn inorder_traversal_t(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }
        let mut stack = Vec::new();
        let mut r = root.clone();

        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }

            r = stack.pop();
            if let Some(node) = r {
                result.push(node.borrow().val);
                r = node.borrow().right.clone();
            }
        }

        result
    }

    // 递归
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = vec![];
        Self::inorder(root, &mut vals);
        return vals;
    }

    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        match root {
            Some(node) => {
                Self::inorder(node.borrow().left.clone(), vals);
                vals.push(node.borrow().val);
                Self::inorder(node.borrow().right.clone(), vals);
            }
            None => return,
        }
    }
}

fn main() {
    let node = TreeNode {
        val: 99,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 98,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 100,
            left: None,
            right: None,
        }))),
    };
    let vals = Solution::inorder_traversal(Some(Rc::new(RefCell::new(node))));
    println!("{:?}", vals)
}
