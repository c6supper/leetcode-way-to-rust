//! 518. Coin Change II
//! Medium
//!
//! You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
//!
//! Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
//!
//! You may assume that you have an infinite number of each kind of coin.
//!
//! The answer is guaranteed to fit into a signed 32-bit integer.
//!
//! Example 1:
//!
//! Input: amount = 5, coins = [1,2,5]
//! Output: 4
//! Explanation: there are four ways to make up the amount:
//! 5=5
//! 5=2+2+1
//! 5=2+1+1+1
//! 5=1+1+1+1+1
//! Example 2:
//!
//! Input: amount = 3, coins = [2]
//! Output: 0
//! Explanation: the amount of 3 cannot be made up just with coins of 2.
//! Example 3:
//!
//! Input: amount = 10, coins = [10]
//! Output: 1
//!
//!
//! Constraints:
//!
//! 1 <= coins.length <= 300
//! 1 <= coins[i] <= 5000
//! All the values of coins are unique.
//! 0 <= amount <= 5000
pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for i in 0..coins.len() {
            for j in coins[i] as usize..dp.len() {
                dp[j] += dp[j - coins[i] as usize];
            }
        }
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::change(10, vec![10]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::change(3, vec![2]));
    }
}
