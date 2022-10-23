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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    false
                } else {
                    Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                        && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
                }
            }
            _ => false,
            // (None, Some(_)) | (Some(_), None) => false,
        }
    }
}

fn main() {
    let p = TreeNode {
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
    let q = TreeNode {
        val: 99,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 98,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 101,
            left: None,
            right: None,
        }))),
    };
    let is_same = Solution::is_same_tree(
        Some(Rc::new(RefCell::new(p))),
        Some(Rc::new(RefCell::new(q))),
    );
    println!("{:?}", is_same)
}
