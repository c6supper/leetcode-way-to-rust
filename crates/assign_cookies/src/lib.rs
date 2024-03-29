//! 455. Assign Cookies
//! Easy
//! Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.
//!
//! Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content with;
//! and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the child i will be content.
//! Your goal is to maximize the number of your content children and output the maximum number.
//!
//! Example 1:
//!
//! Input: g = [1,2,3], s = [1,1]
//! Output: 1
//! Explanation: You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3.
//! And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content.
//! You need to output 1.
//! Example 2:
//!
//! Input: g = [1,2], s = [1,2,3]
//! Output: 2
//! Explanation: You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2.
//! You have 3 cookies and their sizes are big enough to gratify all of the children,
//! You need to output 2.
//!  
//!
//! Constraints:
//!
//! 1 <= g.length <= 3 * 104
//! 0 <= s.length <= 3 * 104
//! 1 <= g[i], s[j] <= 231 - 1

pub struct Solution {}

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut sorted_g = g;
        let mut sorted_s = s;
        let mut contented_num = 0;
        sorted_g.sort();
        sorted_s.sort();
        let mut g_iter = sorted_g.iter();
        let mut s_iter = sorted_s.iter();
        let mut g_e = g_iter.next();
        let mut s_e = s_iter.next();

        loop {
            match (g_e, s_e) {
                (Some(g), Some(s)) => {
                    if s >= g {
                        g_e = g_iter.next();
                        contented_num += 1
                    }
                    s_e = s_iter.next();
                }
                _ => break,
            }
        }
        contented_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
        );
    }
}
