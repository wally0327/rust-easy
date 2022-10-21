use std::ptr::null;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            // 这一步很关键，通过 match 语法将不可变的 list，编程可变的
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    let n = l1.next.take();
                    l1.next = Self::merge_two_lists(n, Some(l2));
                    Some(l1)
                } else {
                    let n = l2.next.take();
                    l2.next = Self::merge_two_lists(n, Some(l1));
                    Some(l2)
                }
            }
            _ => None,
        }
    }
}

fn main() {}
