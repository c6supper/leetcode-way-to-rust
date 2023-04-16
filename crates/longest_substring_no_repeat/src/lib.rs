//* 3. Longest Substring Without Repeating Characters
//* Medium
//* 
//* Given a string s, find the length of the longest substring without repeating characters.
//* 
//*  
//* 
//* Example 1:
//* 
//* Input: s = "abcabcbb"
//* Output: 3
//* Explanation: The answer is "abc", with the length of 3.
//* Example 2:
//* 
//* Input: s = "bbbbb"
//* Output: 1
//* Explanation: The answer is "b", with the length of 1.
//* Example 3:
//* 
//* Input: s = "pwwkew"
//* Output: 3
//* Explanation: The answer is "wke", with the length of 3.
//* Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//*  
//* 
//* Constraints:
//* 
//* 0 <= s.length <= 5 * 104
//* s consists of English letters, digits, symbols and spaces.

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left,mut right) = (0,0);
        let mut sub_map = std::collections::BTreeMap::new();
        let mut max_lens = 0;

        for (i,c) in s.chars().enumerate() {
            while sub_map.contains_key(&c) {
                sub_map.pop_first();
            }
            sub_map.insert(c, i);
            right = i;
            if right - left > max_lens {
                max_lens = right - left;
            }
        }

        max_lens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn test_1() {
//         assert_eq!(5, Solution::minimize_array_value(vec![3, 7, 1, 6]));
//     }
// }
