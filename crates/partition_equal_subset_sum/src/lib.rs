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
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_partition(vec![1,5,11,5]));
    }

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::can_partition(vec![1,2,3,5]));
    }
}
