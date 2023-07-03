//! 71. Simplify Path
//! Medium
//! Companies
//! Given a string path, which is an absolute path (starting with a slash '/') to a file or directory in a Unix-style file system, convert it to the simplified canonical path.
//!
//! In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single slash '/'. For this problem, any other format of periods such as '...' are treated as file/directory names.
//!
//! The canonical path should have the following format:
//!
//! The path starts with a single slash '/'.
//! Any two directories are separated by a single slash '/'.
//! The path does not end with a trailing '/'.
//! The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
//! Return the simplified canonical path.
//!
//!
//!
//! Example 1:
//!
//! Input: path = "/home/"
//! Output: "/home"
//! Explanation: Note that there is no trailing slash after the last directory name.
//! Example 2:
//!
//! Input: path = "/../"
//! Output: "/"
//! Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
//! Example 3:
//!
//! Input: path = "/home//foo/"
//! Output: "/home/foo"
//! Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
//!
//!
//! Constraints:
//!
//! 1 <= path.length <= 3000
//! path consists of English letters, digits, period '.', slash '/' or '_'.
//! path is a valid absolute Unix path.

pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![""];
        for current in path.split('/') {
            match current {
                ".." => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                    continue;
                }
                "." | "" => {
                    continue;
                }
                _ => stack.push(current),
            }
        }

        if stack.len() == 1 {
            return "/".to_string();
        }
        stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("/home"),
            Solution::simplify_path(String::from("/home/"))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            String::from("/"),
            Solution::simplify_path(String::from("/../"))
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            String::from("/home/foo"),
            Solution::simplify_path(String::from("/home//foo/"))
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            String::from("/c"),
            Solution::simplify_path(String::from("/a/./b/../../c/"))
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            String::from("/..."),
            Solution::simplify_path(String::from("/..."))
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            String::from("/"),
            Solution::simplify_path(String::from("/.."))
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            String::from("/a"),
            Solution::simplify_path(String::from("/a/."))
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            String::from("/home/bar"),
            Solution::simplify_path(String::from("/home/foo/./.././bar"))
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            String::from("/a/b/c"),
            Solution::simplify_path(String::from("/a//b////c/d//././/.."))
        );
    }
}
