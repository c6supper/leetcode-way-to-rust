//! 257. Binary Tree Paths
//! Easy
//! Given the root of a binary tree, return all root-to-leaf paths in any order.
//!
//! A leaf is a node with no children.

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
//! Input: root = [1,2,3,null,5]
//! Output: ["1->2->5","1->3"]
//!
//! Example 2:
//!
//! Input: root = [1]
//! Output: ["1"]
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [1, 100].
//! -100 <= Node.val <= 100
//!

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
    pub fn binary_tree_paths(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<String> {
    }

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
    fn test_binary_tree_paths_1() {
        assert_eq!(
            vec!["1->2->5", "1->3"],
            Solution::binary_tree_paths(
                Solution::construct_by_preorder_traverse(&mut vec![
                    1,
                    2,
                    i32::MIN,
                    5,
                    i32::MIN,
                    i32::MIN,
                    3,
                    i32::MIN,
                    i32::MIN
                ])
            )
        );
    }

    fn test_binary_tree_paths_2() {
        assert_eq!(
            vec!["1"],
            Solution::binary_tree_paths(
                Solution::construct_by_preorder_traverse(&mut vec![
                    1,
                    i32::MIN,
                    i32::MIN
                ])
            )
        );
    }
}
