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
    pub fn preorder_traverse_recursively(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        match node {
            None => (),
            Some(n) => {
                ordered_bst.push(n.borrow().val);
                ordered_bst.append(
                    &mut Solution::preorder_traverse_recursively(
                        n.borrow().left.clone(),
                    ),
                );
                ordered_bst.append(
                    &mut Solution::preorder_traverse_recursively(
                        n.borrow().right.clone(),
                    ),
                );
            }
        }
        ordered_bst
    }

    pub fn preorder_traverse_iteratively(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        stack.push(root);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node {
                None => (),
                Some(n) => {
                    ordered_bst.push(n.borrow().val);

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
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        let mut node: Option<Rc<RefCell<TreeNode>>> = root;

        while node.is_some() || !stack.is_empty() {
            match node {
                None => {
                    node = Some(stack.pop().unwrap());
                    if let Some(n) = node {
                        ordered_bst.push(n.borrow().val);
                        node = n.borrow().right.clone();
                    }
                }
                Some(n) => {
                    stack.push(n.clone());
                    node = n.borrow().left.clone();
                }
            }
        }

        ordered_bst
    }

    pub fn inorder_traverse_recursively(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        match node {
            None => (),
            Some(n) => {
                ordered_bst.append(
                    &mut Solution::inorder_traverse_recursively(
                        n.borrow().left.clone(),
                    ),
                );
                ordered_bst.push(n.borrow().val);
                ordered_bst.append(
                    &mut Solution::inorder_traverse_recursively(
                        n.borrow().right.clone(),
                    ),
                );
            }
        }
        ordered_bst
    }

    pub fn postorder_traverse_recursively(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ordered_bst: Vec<i32> = Vec::new();
        match node {
            None => (),
            Some(n) => {
                ordered_bst.append(
                    &mut Solution::postorder_traverse_recursively(
                        n.borrow().left.clone(),
                    ),
                );
                ordered_bst.append(
                    &mut Solution::postorder_traverse_recursively(
                        n.borrow().right.clone(),
                    ),
                );
                ordered_bst.push(n.borrow().val);
            }
        }
        ordered_bst
    }

    pub fn postorder_traverse_iteratively(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut pre_stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut post_stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        if root.is_none() {
            return vec![];
        }

        pre_stack.push(root.unwrap());

        while !pre_stack.is_empty() {
            let node: Option<Rc<RefCell<TreeNode>>> = pre_stack.pop();

            if let Some(n) = node {
                if n.borrow().right.is_some() {
                    pre_stack.push(n.borrow().right.clone().unwrap());
                }
                if n.borrow().left.is_some() {
                    pre_stack.push(n.borrow().left.clone().unwrap());
                }
                post_stack.push(n);
            }
        }

        post_stack.iter().rev().map(|x| x.borrow().val).collect()
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
    fn test_preorder_traverse_recursively_1() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::preorder_traverse_recursively(
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
    fn test_preorder_traverse_recursively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::preorder_traverse_recursively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_preorder_traverse_recursively_3() {
        assert_eq!(
            vec![1],
            Solution::preorder_traverse_recursively(
                Solution::construct_by_preorder_traverse(&mut vec![1])
            )
        );
    }

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
    fn test_inorder_traverse_recursively_1() {
        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traverse_recursively(
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
    fn test_inorder_traverse_recursively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::inorder_traverse_recursively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_inorder_traverse_recursively_3() {
        assert_eq!(
            vec![1],
            Solution::inorder_traverse_recursively(
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
    fn test_postorder_traverse_recursively_1() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::postorder_traverse_recursively(
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
    fn test_postorder_traverse_recursively_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::postorder_traverse_recursively(
                Solution::construct_by_preorder_traverse(&mut vec![])
            )
        );
    }

    #[test]
    fn test_postorder_traverse_recursively_3() {
        assert_eq!(
            vec![1],
            Solution::postorder_traverse_recursively(
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
