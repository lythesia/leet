/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
 
Example 1:
Input: root = [3,9,20,null,null,15,7]
Output: [[3],[20,9],[15,7]]

Example 2:
Input: root = [1]
Output: [[1]]

Example 3:
Input: root = []
Output: []

 
Constraints:

	The number of nodes in the tree is in the range [0, 2000].
	-100 <= Node.val <= 100

 */
pub struct Solution {}
use super::super::util::tree::{TreeNode, to_tree};

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        if let Some(root) = root {
            let mut q = VecDeque::new();
            q.push_back(root);
            let mut rev = false;
            while !q.is_empty() {
                let mut n = q.len();
                let mut v = vec![];
                while let Some(front) = q.pop_front() {
                    v.push(front.borrow().val);
                    let left = front.borrow_mut().left.take();
                    let right = front.borrow_mut().right.take();
                    left.map(|n| q.push_back(n));
                    right.map(|n| q.push_back(n));
                    n -= 1;
                    if n == 0 { break; }
                }
                if rev { v.reverse(); }
                rev = !rev;
                ans.push(v);
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
