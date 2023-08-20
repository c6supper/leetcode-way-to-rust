//! 123. Best Time to Buy and Sell Stock III
//! Hard
//!
//! You are given an array prices where prices[i] is the price of a given stock on the ith day.
//!
//! Find the maximum profit you can achieve. You may complete at most two transactions.
//!
//! Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
//!
//! Example 1:
//!
//! Input: prices = [3,3,5,0,0,3,1,4]
//! Output: 6
//! Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//! Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
//! Example 2:
//!
//! Input: prices = [1,2,3,4,5]
//! Output: 4
//! Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//! Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
//! Example 3:
//!
//! Input: prices = [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.
//!  
//!
//! Constraints:
//!
//! 1 <= prices.length <= 105
//! 0 <= prices[i] <= 105

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 4]; prices.len()];
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        dp[0][2] = -prices[0];
        dp[0][3] = 0;

        for i in 1..prices.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], -prices[i]);
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
            dp[i][2] = std::cmp::max(dp[i - 1][1] - prices[i], dp[i - 1][2]);
            dp[i][3] = std::cmp::max(dp[i - 1][2] + prices[i], dp[i - 1][3]);
        }
        dp[prices.len() - 1][3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::max_profit(vec![1, 2]));
    }
}
