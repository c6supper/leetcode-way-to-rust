//! You are given an array of binary strings strs and two integers m and n.
//!
//! Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
//!
//! A set x is a subset of a set y if all elements of x are also elements of y.
//!
//!  
//! Example 1:
//!
//! Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
//! Output: 4
//! Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
//! Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
//! {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
//! Example 2:
//!
//! Input: strs = ["10","0","1"], m = 1, n = 1
//! Output: 2
//! Explanation: The largest subset is {"0", "1"}, so the answer is 2.
//!  
//!
//! Constraints:
//!
//! 1 <= strs.length <= 600
//! 1 <= strs[i].length <= 100
//! strs[i] consists only of digits '0' and '1'.
//! 1 <= m, n <= 100

pub struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

        for i in 0..strs.len() {
            let zero_num = strs[i].matches('0').count();
            let one_num = strs[i].matches('1').count();
            let mut j = m;
            while j >= zero_num as i32 {
                let mut k = n;
                while k >= one_num as i32 {
                    dp[j as usize][k as usize] = std::cmp::max(
                        dp[j as usize][k as usize],
                        dp[j as usize - zero_num][k as usize - one_num] + 1,
                    );
                    k -= 1;
                }
                j -= 1;
            }
        }

        dp[m as usize][n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::find_max_form(
                vec!["10".to_string(), "0".to_string(), "1".to_string()],
                1,
                1
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            )
        );
    }
}
