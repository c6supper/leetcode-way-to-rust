/// 2439. Minimize Maximum of Array
/// Medium
/// 1.2K
/// 296
/// Companies
/// You are given a 0-indexed array nums comprising of n non-negative integers.
///
/// In one operation, you must:
///
/// Choose an integer i such that 1 <= i < n and nums[i] > 0.
/// Decrease nums[i] by 1.
/// Increase nums[i - 1] by 1.
/// Return the minimum possible value of the maximum integer of nums after performing any number of operations.
///
///  
///
/// Example 1:
///
/// Input: nums = [3,7,1,6]
/// Output: 5
/// Explanation:
/// One set of optimal operations is as follows:
/// 1. Choose i = 1, and nums becomes [4,6,1,6].
/// 2. Choose i = 3, and nums becomes [4,6,2,5].
/// 3. Choose i = 1, and nums becomes [5,5,2,5].
/// The maximum integer of nums is 5. It can be shown that the maximum number cannot be less than 5.
/// Therefore, we return 5.
/// Example 2:
///
/// Input: nums = [10,1]
/// Output: 10
/// Explanation:
/// It is optimal to leave nums as is, and since 10 is the maximum value, we return 10.
///  
///
/// Constraints:
///
/// n == nums.length
/// 2 <= n <= 105
/// 0 <= nums[i] <= 109

pub struct Solution {}

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let (mut prefix_sum, mut min_max) = (0, 0);
        for (i, num) in nums.iter().enumerate() {
            prefix_sum = prefix_sum + num;
            if (*num > min_max) {
                min_max =
                    (prefix_sum + *num) / (i + 1) + (*num > prefix_sum / i).;
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
            16,
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
