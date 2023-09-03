//! 144. Binary Tree Preorder Traversal
//! Easy
//!
//! Given the root of a binary tree, return the preorder traversal of its nodes' values.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example1", "resource/preorder_1.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 1:
//! ![Example 1][example1]
//! Input: root = [1,null,2,3]
//! Output: [1,2,3]
//!
//! Example 2:
//!
//! Input: root = []
//! Output: []
//!
//! Example 3:
//!
//! Input: root = [1]
//! Output: [1]
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [0, 100].
//! -100 <= Node.val <= 100
//!
//! Follow up: Recursive solution is trivial, could you do it iteratively?

pub struct Solution {}

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
impl Solution {
    pub fn pre_order_traverse(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        match node {
            None => (),
            Some(n) => {
                ordered_bst.push(n.borrow().val);
                ordered_bst.append(&mut Solution::pre_order_traverse(
                    n.borrow().left.clone(),
                ));
                ordered_bst.append(&mut Solution::pre_order_traverse(
                    n.borrow().right.clone(),
                ));
            }
        }
        ordered_bst
    }

    pub fn construct_by_pre_order_traverse(
        pre_order_vec: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_order_vec.len() < 1 {
            return None;
        }
        let value = pre_order_vec.remove(0);

        if value == i32::MIN {
            return None;
        } else {
            let node: Option<Rc<RefCell<TreeNode>>> =
                Some(Rc::new(RefCell::new(TreeNode::new(value))));
            node.as_ref().unwrap().borrow_mut().left =
                Solution::construct_by_pre_order_traverse(pre_order_vec);
            node.as_ref().unwrap().borrow_mut().right =
                Solution::construct_by_pre_order_traverse(pre_order_vec);

            node
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::pre_order_traverse(
                Solution::construct_by_pre_order_traverse(&mut vec![
                    1,
                    i32::MIN,
                    2,
                    3
                ])
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::pre_order_traverse(
                Solution::construct_by_pre_order_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![1],
            Solution::pre_order_traverse(
                Solution::construct_by_pre_order_traverse(&mut vec![1])
            )
        );
    }
}
