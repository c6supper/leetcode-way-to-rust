pub struct Solution {}

impl Solution {
    pub fn knapsack(weight: Vec<u32>, value: Vec<u32>, capacity: u32) -> u32 {
        let mut dp: Vec<u32> = vec![0; usize::try_from(capacity + 1).unwrap()];

        for i in 0..weight.len() {
            let mut j = usize::try_from(capacity).unwrap();
            while j >= usize::try_from(weight[i]).unwrap() {
                dp[j] = std::cmp::max(
                    dp[j],
                    dp[j - usize::try_from(weight[i]).unwrap()] + value[i],
                );
                j -= 1
            }
        }

        for (cap, max) in dp.iter().enumerate() {
            println!("knapsack cap={cap}, max value={max}");
        }
        dp[usize::try_from(capacity).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(35, Solution::knapsack(vec![1, 3, 4], vec![15, 20, 30], 4));
    }
}
