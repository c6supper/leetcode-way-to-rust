//! 3. Longest Substring Without Repeating Characters
//! Medium
//!
//! Given a string s, find the length of the longest substring without repeating characters.
//!
//!
//!
//! Example 1:
//!
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//! Example 2:
//!
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//! Example 3:
//!
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!
//!
//! Constraints:
//!
//! 0 <= s.length <= 5 * 104
//! s consists of English letters, digits, symbols and spaces.

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut right;
        let mut sub_map = vec![];
        let mut max_lens: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            if cfg!(test) {
                println!("for index = {}, value = {}", &i, &c);
            }

            if sub_map.contains(&c) {
                loop {
                    let e = sub_map.remove(0);
                    if cfg!(test) {
                        println!("pop index = {}, value = {}", &left, &e);
                    }
                    left += 1;
                    if e == c {
                        // left = e.1 + 1;
                        break;
                    }
                }
            }

            sub_map.push(c);
            if cfg!(test) {
                println!("push index = {}, value = {}", &i, &c);
            }
            right = i + 1;
            max_lens = std::cmp::max(max_lens, (right - left) as i32);

            if cfg!(test) {
                dbg!(&sub_map);
            }
        }

        max_lens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }
}
