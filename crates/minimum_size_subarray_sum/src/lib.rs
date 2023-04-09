//! 209. Minimum Size Subarray Sum
//!
//! Given an array of positive integers nums and a positive integer target, return the minimal length of a
//! subarray
//!  whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
//!
//!  
//!
//! Example 1:
//!
//! Input: target = 7, nums = [2,3,1,2,4,3]
//! Output: 2
//! Explanation: The subarray [4,3] has the minimal length under the problem constraint.
//! Example 2:
//!
//! Input: target = 4, nums = [1,4,4]
//! Output: 1
//! Example 3:
//!
//! Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//! Output: 0
//!  
//!
//! Constraints:
//!
//! 1 <= target <= 109
//! 1 <= nums.length <= 105
//! 1 <= nums[i] <= 104
//!  
//!
//! Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).

pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = i32::MAX;
        let mut sum: i32 = 0;
        let (mut left_idx, mut right_idx) = (0, 0);

        while right_idx < nums.len() {
            sum += nums[right_idx];
            while target <= sum {
                min_len = min_len.min(right_idx as i32 - left_idx as i32 + 1);
                sum -= nums[left_idx];
                left_idx += 1;
            }
            right_idx += 1;
        }

        if min_len == i32::MAX {
            0
        } else {
            min_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::min_sub_array_len(4, vec![1, 4, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
