//! 337. House Robber III
//! Medium

//! The thief has found himself a new place for his thievery again. There is only one entrance to this area, called root.
//!
//! Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that all houses in this place form a binary tree. It will automatically contact the police if two directly-linked houses were broken into on the same night.
//!
//! Given the root of the binary tree, return the maximum amount of money the thief can rob without alerting the police.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example1", "resource/rob1-tree.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 1:
//! ![Example 1][example1]
//!
//! Input: root = [3,2,3,null,3,null,1]
//! Output: 7
//! Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
//!
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example2", "resource/rob2-tree.jpg")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 2:
//! ![Example 2][example2]

//!
//! Input: root = [3,4,5,1,3,null,1]
//! Output: 9
//! Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
//!  
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [1, 104].
//! 0 <= Node.val <= 104

pub struct Solution {}

impl Solution {
    pub fn knapsack(weight: Vec<u32>, value: Vec<u32>, capacity: u32) -> u32 {
        let mut dp: Vec<u32> = vec![0; usize::try_from(capacity + 1).unwrap()];

        for i in 0..weight.len() {
            let mut j = usize::try_from(capacity).unwrap();
            while j >= usize::try_from(weight[i]).unwrap() {
                dp[j] = std::cmp::max(
                    dp[j],
                    dp[j - usize::try_from(weight[i]).unwrap()] + value[i],
                );
                j -= 1
            }
        }

        for (cap, max) in dp.iter().enumerate() {
            println!("knapsack cap={cap}, max value={max}");
        }
        dp[usize::try_from(capacity).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(35, Solution::knapsack(vec![1, 3, 4], vec![15, 20, 30], 4));
    }
}
