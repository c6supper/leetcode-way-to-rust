//! 1049. Last Stone Weight II
//! Medium
//!
//! You are given an array of integers stones where stones[i] is the weight of the ith stone.
//!
//! We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights x and y with x <= y. The result of this smash is:
//!
//! If x == y, both stones are destroyed, and
//! If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
//! At the end of the game, there is at most one stone left.
//!
//! Return the smallest possible weight of the left stone. If there are no stones left, return 0.
//!
//!
//!
//! Example 1:
//!
//! Input: stones = [2,7,4,1,8,1]
//! Output: 1
//! Explanation:
//! We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
//! we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
//! we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
//! we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
//! Example 2:
//!
//! Input: stones = [31,26,33,21,40]
//! Output: 5
//!
//!
//! Constraints:
//!
//! 1 <= stones.length <= 30
//! 1 <= stones[i] <= 100

use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let capacity: usize =
            usize::try_from(stones.iter().sum::<i32>() / 2).unwrap();
        let mut dp = vec![0; capacity + 1];
        for i in 0..stones.len() {
            let mut j: usize = capacity;
            while j >= usize::try_from(stones[i]).unwrap() {
                dp[j] = max(
                    dp[j],
                    dp[j - usize::try_from(stones[i]).unwrap()] + stones[i],
                );
                j -= 1
            }
        }
        for (cap, max) in dp.iter().enumerate() {
            println!("knapsack cap={cap}, max value={max}");
        }
        stones.iter().sum::<i32>() - 2 * dp[capacity]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::last_stone_weight_ii(vec![31, 31]));
    }
}
