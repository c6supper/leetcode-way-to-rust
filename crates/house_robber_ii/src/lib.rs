//! 213. House Robber II
//! Medium
//!
//! You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle.
//! That means the first house is the neighbor of the last one.
//! Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
//!
//! Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
//!
//!  
//!
//! Example 1:
//! Input: nums = [2,3,2]
//! Output: 3
//! Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
//!
//!
//! Example 2:
//! Input: nums = [1,2,3,1]
//! Output: 4
//! Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//! Total amount you can rob = 1 + 3 = 4.
//!
//! Example 3:
//! Input: nums = [1,2,3]
//! Output: 3
//!  
//! Constraints:
//!
//! 1 <= nums.length <= 100
//! 0 <= nums[i] <= 1000

pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        dp[1] = nums[0];

        for i in 2..dp.len() {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i - 1], dp[i - 1]);
        }
        let left_max = dp[dp.len() - 1];

        if nums.len() < 2 {
            return left_max;
        }

        let mut dp = vec![0; nums.len() + 1];
        dp[2] = nums[1];
        for i in 3..dp.len() {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i - 1], dp[i - 1]);
        }

        if left_max > dp[dp.len() - 1] {
            left_max
        } else {
            dp[dp.len() - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::rob(vec![1, 2, 3]));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, Solution::rob(vec![2]));
    }
}
