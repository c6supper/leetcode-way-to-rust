//! Perfect Squares
//! Medium
//!
//! Given an integer n, return the least number of perfect square numbers that sum to n.
//!
//! A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
//!
//!  
//!
//! Example 1:
//!
//! Input: n = 12
//! Output: 3
//! Explanation: 12 = 4 + 4 + 4.
//!
//! Example 2:
//!
//! Input: n = 13
//! Output: 2
//! Explanation: 13 = 4 + 9.
//!  
//!
//! Constraints:
//!
//! 1 <= n <= 104

pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let root = (n as f32).sqrt() as i32 + 1;
        let mut dp = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;

        for i in 1..root {
            for j in i * i..n + 1 {
                dp[j as usize] =
                    std::cmp::min(dp[j as usize], dp[(j - i * i) as usize] + 1);
            }
        }

        // if cfg!(test) {
        //     println!("n = {n}, root = {root}");
        //     for (cap, count) in dp.iter().enumerate() {
        //         println!("cap = {cap}, count = {count}");
        //     }
        // }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
