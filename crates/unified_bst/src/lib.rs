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
//!
//! 94. Binary Tree Inorder Traversal
//! Easy
//! Given the root of a binary tree, return the inorder traversal of its nodes' values.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example2", "resource/inorder_1.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 1:
//! ![Example 1][example2]
//! Input: root = [1,null,2,3]
//! Output: [1,3,2]
//! Example 2:
//!
//! Input: root = []
//! Output: []
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
//!
//! 145. Binary Tree Postorder Traversal
//! Easy
//! Given the root of a binary tree, return the postorder traversal of its nodes' values.
//!
//!  
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example3", "resource/inorder_1.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 1:
//! ![Example 1][example3]
//! Input: root = [1,null,2,3]
//! Output: [3,2,1]
//! Example 2:
//!
//! Input: root = []
//! Output: []
//! Example 3:
//!
//! Input: root = [1]
//! Output: [1]
//!  
//!
//! Constraints:
//!
//! The number of the nodes in the tree is in the range [0, 100].
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
    pub fn preorder_traverse_iteratively(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        stack.push(root);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node {
                None => {
                    let node = stack.pop().unwrap();
                    ordered_bst.push(node.unwrap().borrow().val);
                }
                Some(n) => {
                    stack.push(Some(n.clone()));
                    stack.push(None);
                    if n.borrow().right.is_some() {
                        stack.push(n.borrow().right.clone());
                    }
                    if n.borrow().left.is_some() {
                        stack.push(n.borrow().left.clone());
                    }
                }
            }
        }

        ordered_bst
    }

    pub fn inorder_traverse_iteratively(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        stack.push(root);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node {
                None => {
                    let node = stack.pop().unwrap();
                    ordered_bst.push(node.unwrap().borrow().val);
                }
                Some(n) => {
                    if n.borrow().right.is_some() {
                        stack.push(n.borrow().right.clone());
                    }
                    stack.push(Some(n.clone()));
                    stack.push(None);
                    if n.borrow().left.is_some() {
                        stack.push(n.borrow().left.clone());
                    }
                }
            }
        }

        ordered_bst
    }

    pub fn postorder_traverse_iteratively(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        stack.push(root);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node {
                None => {
                    let node = stack.pop().unwrap();
                    ordered_bst.push(node.unwrap().borrow().val);
                }
                Some(n) => {
                    if n.borrow().right.is_some() {
                        stack.push(n.borrow().right.clone());
                    }
                    if n.borrow().left.is_some() {
                        stack.push(n.borrow().left.clone());
                    }
                    stack.push(Some(n.clone()));
                    stack.push(None);
                }
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
    fn test_preorder_traverse_iteratively_1() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::preorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![
                    1,
                    i32::MIN,
                    2,
                    3
                ])
            )
        );
    }

    #[test]
    fn test_preorder_traverse_iteratively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::preorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_preorder_traverse_iteratively_3() {
        assert_eq!(
            vec![1],
            Solution::preorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![1])
            )
        );
    }

    #[test]
    fn test_inorder_traverse_iteratively_1() {
        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![
                    1,
                    i32::MIN,
                    2,
                    3
                ])
            )
        );
    }

    #[test]
    fn test_inorder_traverse_iteratively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::inorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_inorder_traverse_iteratively_3() {
        assert_eq!(
            vec![1],
            Solution::inorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![1])
            )
        );
    }

    #[test]
    fn test_postorder_traverse_iteratively_1() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::postorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![
                    1,
                    i32::MIN,
                    2,
                    3
                ])
            )
        );
    }

    #[test]
    fn test_postorder_traverse_iteratively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::postorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_postorder_traverse_iteratively_3() {
        assert_eq!(
            vec![1],
            Solution::postorder_traverse_iteratively(
                Solution::construct_by_preorder_traverse(&mut vec![1])
            )
        );
    }
}
