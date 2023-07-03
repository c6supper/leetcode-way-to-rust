//! 377. Combination Sum IV
//! Medium
//! Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
//!
//! The test cases are generated so that the answer can fit in a 32-bit integer.
//!
//! Example 1:
//!
//! Input: nums = [1,2,3], target = 4
//! Output: 7
//! Explanation:
//! The possible combination ways are:
//! (1, 1, 1, 1)
//! (1, 1, 2)
//! (1, 2, 1)
//! (1, 3)
//! (2, 1, 1)
//! (2, 2)
//! (3, 1)
//! Note that different sequences are counted as different combinations.
//! Example 2:
//!
//! Input: nums = [9], target = 3
//! Output: 0
//!
//!
//! Constraints:
//!
//! 1 <= nums.length <= 200
//! 1 <= nums[i] <= 1000
//! All the elements of nums are unique.
//! 1 <= target <= 1000
//!
//!
//! Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?

pub struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        for j in 1 as usize..dp.len() {
            for i in 0..nums.len() {
                if j >= nums[i] as usize {
                    dp[j] += dp[j - nums[i] as usize];
                }
            }
        }

        if cfg!(test) {
            for (cap, v) in dp.iter().enumerate() {
                println!("knapsack cap={cap}, value={v}");
            }
        }

        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::combination_sum4(vec![9], 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(7, Solution::combination_sum4(vec![1, 2, 3], 4));
    }
}
