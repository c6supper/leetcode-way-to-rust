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

pub struct Solution {}

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let (mut prefix_sum, mut min_max) = (0u64, 0i32);
        for (i, num) in nums.iter().enumerate() {
            prefix_sum = prefix_sum + *num as u64;
            let cur_max = ((prefix_sum + i as u64) / (i as u64 + 1)) as i32;
            if cur_max > min_max {
                min_max = cur_max;
            }
        }
        min_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::minimize_array_value(vec![3, 7, 1, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(10, Solution::minimize_array_value(vec![10, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            16,
            Solution::minimize_array_value(vec![13, 13, 20, 0, 8, 9, 9])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            554808881,
            Solution::minimize_array_value(vec![
                153096409, 343881218, 741913853, 343594218, 864722890,
                354938680, 279386271, 616365038, 896106991, 540459582,
                124304477, 856321779, 533947835, 590383040, 708653960,
                928865842, 501462358, 113265076, 786991139, 83872665,
                314304738, 719655858, 685019739, 773289565, 224287062,
                961183249, 922185605, 437586814, 431957201, 622418600
            ])
        );
    }
}
