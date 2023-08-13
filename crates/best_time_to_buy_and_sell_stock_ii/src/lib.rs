pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {}
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
        assert_eq!(7, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(7, Solution::max_profit(vec![7]));
    }
}
