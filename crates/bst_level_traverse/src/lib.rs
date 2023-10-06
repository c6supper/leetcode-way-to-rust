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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut order_bst = vec![];
        let mut queue = Vec::new();

        if root.is_none() {
            return order_bst;
        }
        queue.push(root);

        while !queue.is_empty() {
            let mut level_size = queue.len();
            let mut sub_queue: Vec<i32> = Vec::new();
            while level_size > 0 {
                let node = queue.remove(0);
                match node {
                    Some(n) => {
                        if n.borrow().left.is_some() {
                            queue.insert(
                                queue.len() as usize,
                                n.borrow().left.clone(),
                            );
                        }
                        if n.borrow().right.is_some() {
                            queue.insert(
                                queue.len() as usize,
                                n.borrow().right.clone(),
                            );
                        }
                        sub_queue
                            .insert(sub_queue.len() as usize, n.borrow().val);
                    }
                    None => (),
                }

                level_size -= 1;
            }
            order_bst.push(sub_queue);
        }

        order_bst
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
    fn test_level_order_1() {
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(Solution::construct_by_preorder_traverse(
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
                    i32::MIN,
                ]
            ))
        );
    }

    #[test]
    fn test_level_order_2() {
        assert_eq!(
            vec![vec![1]],
            Solution::level_order(Solution::construct_by_preorder_traverse(
                &mut vec![1]
            ))
        );
    }

    #[test]
    fn test_level_order_3() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::level_order(Solution::construct_by_preorder_traverse(
                &mut vec![]
            ))
        );
    }
}
