//! You are given an integer array nums and an integer target.
//!
//! You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
//!
//! For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
//! Return the number of different expressions that you can build, which evaluates to target.
//!
//! Example 1:
//!
//! Input: nums = [1,1,1,1,1], target = 3
//! Output: 5
//! Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
//! -1 + 1 + 1 + 1 + 1 = 3
//! +1 - 1 + 1 + 1 + 1 = 3
//! +1 + 1 - 1 + 1 + 1 = 3
//! +1 + 1 + 1 - 1 + 1 = 3
//! +1 + 1 + 1 + 1 - 1 = 3
//! Example 2:
//!
//! Input: nums = [1], target = 1
//! Output: 1
//!
//!
//! Constraints:
//!
//! 1 <= nums.length <= 20
//! 0 <= nums[i] <= 1000
//! 0 <= sum(nums[i]) <= 1000
//! -1000 <= target <= 1000

pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        if (nums.iter().sum::<i32>() + target) % 2 != 0
            || (nums.iter().sum::<i32>() + target) < 0
        {
            return 0;
        }
        let capacity: usize = (nums.iter().sum::<i32>() + target) as usize / 2;
        let mut dp = vec![0; capacity + 1];
        dp[0] = 1;

        for i in 0..nums.len() as usize {
            let mut j = capacity as i32;
            while j >= nums[i] {
                dp[j as usize] += dp[(j - nums[i]) as usize];
                j -= 1;
            }
        }
        dp[capacity]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::find_target_sum_ways(vec![1], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 99));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::find_target_sum_ways(vec![1, 0], 2));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, Solution::find_target_sum_ways(vec![100], -200));
    }
}
