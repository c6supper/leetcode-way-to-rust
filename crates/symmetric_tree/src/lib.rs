//! 102. Binary Tree Level Order Traversal
//! Medium
//!
//! Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
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
//! Output: [[3],[9,20],[15,7]]
//! Example 2:
//!
//! Input: root = [1]
//! Output: [[1]]
//! Example 3:
//!
//! Input: root = []
//! Output: []
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [0, 2000].
//! -1000 <= Node.val <= 1000

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            return Solution::bst_left_traversal(node.borrow().left.clone())
                .eq(&Solution::bst_right_traversal(
                    node.borrow().right.clone(),
                ));
        } else {
            true
        }
    }

    pub fn bst_left_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut ordered_bst = Vec::new();

        if root.is_none() {
            return ordered_bst;
        }

        stack.push(root);

        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                stack.push(Some(node.clone()));
                stack.push(None);

                if node.borrow().right.is_some() {
                    stack.push(node.borrow().right.clone());
                }
                if node.borrow().left.is_some() {
                    stack.push(node.borrow().left.clone());
                }
            } else {
                let node = stack.pop().unwrap();
                ordered_bst.push(node.unwrap().borrow().val);
            }
        }

        ordered_bst
    }

    pub fn bst_right_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut ordered_bst = Vec::new();

        if root.is_none() {
            return ordered_bst;
        }

        stack.push(root);

        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                stack.push(Some(node.clone()));
                stack.push(None);

                if node.borrow().left.is_some() {
                    stack.push(node.borrow().left.clone());
                }

                if node.borrow().right.is_some() {
                    stack.push(node.borrow().right.clone());
                }
            } else {
                let node = stack.pop().unwrap();
                ordered_bst.push(node.unwrap().borrow().val);
            }
        }

        ordered_bst
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
    fn test_is_symmetric_1() {
        assert_eq!(
            true,
            Solution::is_symmetric(Solution::construct_by_preorder_traverse(
                &mut vec![
                    1,
                    2,
                    3,
                    i32::MIN,
                    i32::MIN,
                    4,
                    i32::MIN,
                    i32::MIN,
                    2,
                    4,
                    i32::MIN,
                    i32::MIN,
                    3,
                    i32::MIN,
                    i32::MIN
                ]
            ))
        );
    }

    #[test]
    fn test_is_symmetric_2() {
        assert_eq!(
            false,
            Solution::is_symmetric(Solution::construct_by_preorder_traverse(
                &mut vec![1, 2, i32::MIN, 3, i32::MIN, 2, i32::MIN, 3]
            ))
        );
    }

    #[test]
    fn test_is_symmetric_3() {
        assert_eq!(
            true,
            Solution::is_symmetric(Solution::construct_by_preorder_traverse(
                &mut vec![]
            ))
        );
    }
}
