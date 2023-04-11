//! 2. Add Two Numbers
//! Medium
//! Companies
//! You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!

#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example1", "resource/addtwonumber1.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! Example 1:
//! ![Example 1][example1]
//!
//!
//! Input: l1 = \[2,4,3\], l2 = \[5,6,4\]
//! Output: \[7,0,8\]
//! Explanation: 342 + 465 = 807.
//! Example 2:
//!
//! Input: l1 = \[0\], l2 = \[0\]
//! Output: \[0\]
//! Example 3:
//!
//! Input: l1 = \[9,9,9,9,9,9,9\], l2 = \[9,9,9,9\]
//! Output: \[8,9,9,9,0,0,0,1\]
//!  
//!
//! Constraints:
//!
//! The number of nodes in each linked list is in the range \[1, 100\].
//! 0 <= Node.val <= 9
//! It is guaranteed that the list represents a number that does not have leading zeros.

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

    #[allow(dead_code)]
    fn build(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut head;
        for num in nums.iter() {
            tail = match tail.as_mut() {
                Some(c) => {
                    c.next = Some(Box::new(ListNode::new(*num)));
                    &mut c.next
                }
                _ => unreachable!(),
            };
        }

        head.unwrap().next
    }
}
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut sum;
        let (mut e1, mut e2) = (&l1, &l2);
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut head;

        while e1.is_some() || e2.is_some() || carry > 0 {
            match (e1, e2) {
                (Some(a), Some(b)) => {
                    sum = a.val + b.val;
                    e1 = &a.next;
                    e2 = &b.next;
                }
                (None, Some(b)) => {
                    sum = b.val;
                    e2 = &b.next;
                }
                (Some(a), None) => {
                    sum = a.val;
                    e1 = &a.next;
                }
                (None, None) => sum = 0,
            }
            sum += carry;
            carry = sum / 10;
            sum %= 10;

            tail = match tail.as_mut() {
                Some(c) => {
                    c.next = Some(Box::new(ListNode::new(sum)));
                    &mut c.next
                }
                _ => unreachable!(),
            };

            println!("{}", sum);
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            ListNode::build(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(
                ListNode::build(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::build(vec![9, 9, 9, 9])
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            ListNode::build(vec![7, 0, 8]),
            Solution::add_two_numbers(
                ListNode::build(vec![2, 4, 3]),
                ListNode::build(vec![5, 6, 4])
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            ListNode::build(vec![0]),
            Solution::add_two_numbers(
                ListNode::build(vec![0]),
                ListNode::build(vec![0])
            )
        );
    }
}
