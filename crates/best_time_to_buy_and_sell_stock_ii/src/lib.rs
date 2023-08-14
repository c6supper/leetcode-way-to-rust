//! 122. Best Time to Buy and Sell Stock II
//! Medium
//!
//! You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
//!
//! On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.
//!
//! Find and return the maximum profit you can achieve.
//!
//! Example 1:
//! Input: prices = [7,1,5,3,6,4]
//! Output: 7
//! Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
//! Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
//! Total profit is 4 + 3 = 7.
//!
//! Example 2:
//! Input: prices = [1,2,3,4,5]
//! Output: 4
//! Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//! Total profit is 4.
//!
//! Example 3:
//! Input: prices = [7,6,4,3,1]
//! Output: 0
//! Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.
//!  
//!
//! Constraints:
//!
//! 1 <= prices.length <= 3 * 104
//! 0 <= prices[i] <= 104

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 2]; prices.len()];

        dp[0][0] = -prices[0];
        dp[0][1] = 0;

        for i in 1..prices.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
        }

        dp[prices.len() - 1][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
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
        assert_eq!(0, Solution::max_profit(vec![7]));
    }
}
