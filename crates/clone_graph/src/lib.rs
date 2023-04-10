//! Given a reference of a node in a connected undirected graph.
//!
//! Return a deep copy (clone) of the graph.
//!
//! Each node in the graph contains a value (int) and a list (List\[Node\]) of its neighbors.
//!
//! class Node {
//!     public int val;
//!     public List\<Node\> neighbors;
//! }
//!  
//!
//! Test case format:
//!
//! For simplicity, each node's value is the same as the node's index (1-indexed). For example, the first node with val == 1, the second node with val == 2, and so on. The graph is represented in the test case using an adjacency list.
//!
//! An adjacency list is a collection of unordered lists used to represent a finite graph. Each list describes the set of neighbors of a node in the graph.
//!
//! The given node will always be the first node with val = 1. You must return the copy of the given node as a reference to the cloned graph.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example1", "resource/133_clone_graph_question.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! Example 1:
//! ![Example 1][example1]
//!
//! Input: adjList = \[\[2,4\],\[1,3\],\[2,4\],\[1,3\]\]
//!
//! Output: \[\[2,4\],\[1,3\],\[2,4\],\[1,3\]\]
//!
//! Explanation: There are 4 nodes in the graph.
//!
//! 1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
//!
//! 2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
//!
//! 3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
//!
//! 4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("example2", "resource/graph.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! Example 2:
//! ![Example 2][example2]
//!
//! Input: adjList = \[\[\]\]
//!
//! Output: \[\[\]\]
//!
//! Explanation: Note that the input contains one empty list. The graph consists of only one node with val = 1 and it does not have any neighbors.
//!
//! Example 3:
//!
//! Input: adjList = \[\]
//!
//! Output: \[\]
//!
//! Explanation: This an empty graph, it does not have any nodes.
//!
//! Constraints:
//!
//! The number of nodes in the graph is in the range \[0, 100\].
//!
//! 1 <= Node.val <= 100
//!
//! Node.val is unique for each node.
//!
//! There are no repeated edges and no self-loops in the graph.
//!
//! The Graph is connected and all nodes can be visited starting from the given node.
//!

pub struct Solution {}

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let (mut prefix_sum, mut min_max) = (0u64, 0i32);
        for (i, num) in nums.iter().enumerate() {
            prefix_sum = prefix_sum + *num as u64;
            let cur_max = ((prefix_sum + i as u64) / (i as u64 + 1)) as i32;
            if cur_max > min_max {
                min_max = cur_max;
            }
        }
        min_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::minimize_array_value(vec![3, 7, 1, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(10, Solution::minimize_array_value(vec![10, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            16,
            Solution::minimize_array_value(vec![13, 13, 20, 0, 8, 9, 9])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            554808881,
            Solution::minimize_array_value(vec![
                153096409, 343881218, 741913853, 343594218, 864722890,
                354938680, 279386271, 616365038, 896106991, 540459582,
                124304477, 856321779, 533947835, 590383040, 708653960,
                928865842, 501462358, 113265076, 786991139, 83872665,
                314304738, 719655858, 685019739, 773289565, 224287062,
                961183249, 922185605, 437586814, 431957201, 622418600
            ])
        );
    }
}
