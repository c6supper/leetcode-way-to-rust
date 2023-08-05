//! 121. Best Time to Buy and Sell Stock
//! Easy
//!
//! You are given an array prices where prices[i] is the price of a given stock on the ith day.
//!
//! You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
//!
//! Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
//!
//! Example 1:
//!
//! Input: prices = [7,1,5,3,6,4]
//! Output: 5
//! Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//! Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
//!
//! Example 2:
//!
//! Input: prices = [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transactions are done and the max profit = 0.
//!  
//!
//! Constraints:
//!
//! 1 <= prices.length <= 105
//! 0 <= prices[i] <= 104

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut max_prices = vec![0; prices.len()];
        let mut max_profit = 0;
        let mut i: i32 = prices.len() as i32 - 2;
        max_prices[prices.len() - 1] = prices[prices.len() - 1];
        while i > -1 {
            max_prices[i as usize] =
                std::cmp::max(max_prices[i as usize + 1], prices[i as usize]);
            max_profit = std::cmp::max(
                max_profit,
                max_prices[i as usize] - prices[i as usize],
            );
            i -= 1;
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::max_profit(vec![7]));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::max_profit(vec![1, 2]));
    }
}
