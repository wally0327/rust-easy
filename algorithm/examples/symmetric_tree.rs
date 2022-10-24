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
    pub fn is_symmetric_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => Self::symmetric(node.borrow().left.clone(), node.borrow().right.clone()),
            None => true,
        }
    }

    pub fn symmetric(
        l_root: Option<Rc<RefCell<TreeNode>>>,
        r_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (l_root, r_root) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val
                    && Self::symmetric(l.borrow().left.clone(), r.borrow().right.clone())
                    && Self::symmetric(l.borrow().right.clone(), r.borrow().left.clone())
            }
            _ => false,
        }
    }

    // solution 2:
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.clone();
                let mut stack = vec![];
                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());

                while !stack.is_empty() {
                    let l = stack.pop().unwrap();
                    let r = stack.pop().unwrap();

                    match (l.clone(), r.clone()) {
                        (Some(l), Some(r)) => {
                            if l.borrow().val != r.borrow().val {
                                return false;
                            } else {
                                stack.push(l.borrow().left.clone());
                                stack.push(r.borrow().right.clone());
                                stack.push(l.borrow().right.clone());
                                stack.push(r.borrow().left.clone());
                                continue;
                            }
                        }
                        (None, None) => (),
                        _ => return false,
                    }
                }
                true
            }
            None => true,
        }
    }
}

fn main() {
    let l = TreeNode {
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

    println!(
        "is symmetric: {}",
        Solution::is_symmetric(Some(Rc::new(RefCell::new(l))))
    );
}
