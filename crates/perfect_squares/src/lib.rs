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
    pub fn num_squares(n: i32) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
