//! 111. Minimum Depth of Binary Tree
//! Easy
//!
//! Given a binary tree, find its minimum depth.
//!
//! The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
//!
//! Note: A leaf is a node with no children.
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
//! Input: root = [3,9,20,null,null,15,7]
//! Output: 2
//!
//! Example 2:
//!
//! Input: root = [2,null,3,null,4,null,5,null,6]
//! Output: 5
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [0, 104].
//! -100 <= Node.val <= 100

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = Vec::new();
        let mut depth = 0;

        if root.is_none() {
            return 0;
        }

        queue.push(root);

        while !queue.is_empty() {
            depth += 1;
            for _ in 0..queue.len() {
                let node = queue.remove(0).unwrap();
                if node.borrow().left.is_none() && node.borrow().right.is_none()
                {
                    return depth;
                }
                if node.borrow().left.is_some() {
                    queue.push(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push(node.borrow().right.clone());
                }
            }
        }

        depth
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
    fn test_min_depth_1() {
        assert_eq!(
            2,
            Solution::min_depth(Solution::construct_by_preorder_traverse(
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
    fn test_min_depth_2() {
        assert_eq!(
            5,
            Solution::min_depth(Solution::construct_by_preorder_traverse(
                &mut vec![
                    2,
                    i32::MIN,
                    3,
                    i32::MIN,
                    4,
                    i32::MIN,
                    5,
                    i32::MIN,
                    6,
                    i32::MIN,
                    i32::MIN
                ]
            ))
        );
    }

    #[test]
    fn test_min_depth_3() {
        assert_eq!(
            0,
            Solution::min_depth(Solution::construct_by_preorder_traverse(
                &mut vec![]
            ))
        );
    }
}
