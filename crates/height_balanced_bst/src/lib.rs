//! 110. Balanced Binary Tree
//! Easy
//!
//! Given a binary tree, determine if it is height-balanced.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example1", "resource/tree1.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 1:
//!
//!
//! Input: root = [3,9,20,null,null,15,7]
//! Output: true
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example2", "resource/balance_2.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 2:
//!
//! Input: root = [1,2,2,3,3,null,null,4,4]
//! Output: false
//! Example 3:
//!
//! Input: root = []
//! Output: true
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [0, 5000].
//! -104 <= Node.val <= 104

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {}

    pub fn construct_by_preorder_traverse(
        preorder_vec: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_vec.len() < 1 {
            return None;
        }
        let value = preorder_vec.remove(0);

        if value == i32::MIN {
            return None;
        } else {
            let node: Option<Rc<RefCell<TreeNode>>> =
                Some(Rc::new(RefCell::new(TreeNode::new(value))));
            node.as_ref().unwrap().borrow_mut().left =
                Solution::construct_by_preorder_traverse(preorder_vec);
            node.as_ref().unwrap().borrow_mut().right =
                Solution::construct_by_preorder_traverse(preorder_vec);

            node
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced_1() {
        assert_eq!(
            true,
            Solution::is_balanced(Solution::construct_by_preorder_traverse(
                &mut vec![
                    3,
                    9,
                    i32::MIN,
                    i32::MIN,
                    20,
                    15,
                    i32::MIN,
                    i32::MIN,
                    7,
                    i32::MIN,
                    i32::MIN
                ]
            ))
        );
    }

    #[test]
    fn test_is_balanced_2() {
        assert_eq!(
            false,
            Solution::is_balanced(Solution::construct_by_preorder_traverse(
                &mut vec![
                    1,
                    2,
                    3,
                    4,
                    i32::MIN,
                    i32::MIN,
                    4,
                    i32::MIN,
                    i32::MIN,
                    3,
                    i32::MIN,
                    i32::MIN,
                    2,
                    i32::MIN,
                    i32::MIN
                ]
            ))
        );
    }

    #[test]
    fn test_is_balanced_3() {
        assert_eq!(
            true,
            Solution::is_balanced(Solution::construct_by_preorder_traverse(
                &mut vec![]
            ))
        );
    }
}
