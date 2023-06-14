//* 416. Partition Equal Subset Sum
//* Medium
//*
//* Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
//*
//*
//*
//* Example 1:
//*
//* Input: nums = [1,5,11,5]
//* Output: true
//* Explanation: The array can be partitioned as [1, 5, 5] and [11].
//* Example 2:
//*
//* Input: nums = [1,2,3,5]
//* Output: false
//* Explanation: The array cannot be partitioned into equal sum subsets.
//*
//*
//* Constraints:
//* 1 <= nums.length <= 200
//* 1 <= nums[i] <= 100

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let half:usize = nums.iter().sum::<i32>() as usize / 2;

        if 2 * half < nums.iter().sum::<i32>() as usize {
            return false;
        }

        let mut dp = vec![0; half as usize + 1];

        for i in 0..nums.len() {
            let mut j = half;
            while j >= nums[i] as usize {
                dp[j] = std::cmp::max(dp[j], dp[j - nums[i] as usize] + nums[i]);
                j -= 1
            }
        }
        for (cap, max) in dp.iter().enumerate() {
            println!("knapsack cap={cap}, max value={max}");
        }
        return half == dp[half as usize] as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
