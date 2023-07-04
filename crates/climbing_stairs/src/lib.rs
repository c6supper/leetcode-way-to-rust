//! 70. Climbing Stairs
//! Easy
//!
//! You are climbing a staircase. It takes n steps to reach the top.
//!
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//!
//! Example 1:
//!
//! Input: n = 2
//! Output: 2
//! Explanation: There are two ways to climb to the top.
//! 1. 1 step + 1 step
//! 2. 2 steps
//! Example 2:
//!
//! Input: n = 3
//! Output: 3
//! Explanation: There are three ways to climb to the top.
//! 1. 1 step + 1 step + 1 step
//! 2. 1 step + 2 steps
//! 3. 2 steps + 1 step

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        dp[0] = 1;
        for j in 1..n + 1 {
            for i in 1..3 {
                if j >= i {
                    dp[j as usize] += dp[(j - i) as usize];
                }
            }
        }

        if cfg!(test) {
            for (cap, max) in dp.iter().enumerate() {
                println!("knapsack cap={cap}, max value={max}");
            }
        }
        dp[usize::try_from(n).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
