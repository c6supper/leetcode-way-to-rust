/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
///  
///
/// Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///  
///
/// Constraints:
///
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// Only one valid answer exists.
///  
///
/// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
use std::{collections::HashMap, vec};

pub struct Solution {}

impl Solution {
    pub fn two_sum_inefficient(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            for ii in i + 1..nums.len() {
                if nums[i] + nums[ii] == target {
                    return vec![i.try_into().unwrap(), ii.try_into().unwrap()];
                }
            }
        }
        vec![]
    }

    pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hashmap = HashMap::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if nums_hashmap.contains_key(&complement) {
                return vec![*nums_hashmap.get(&complement).unwrap(), i as i32];
            }
            nums_hashmap.insert(num, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 1],
            Solution::two_sum_inefficient(&vec![2, 7, 11, 15], 9)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2],
            Solution::two_sum_inefficient(&vec![3, 2, 4], 6)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], Solution::two_sum(&vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![1, 2], Solution::two_sum(&vec![3, 2, 4], 6));
    }
}
