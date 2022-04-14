/**
 * [199] Binary Tree Right Side View
 *
 * Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 
Example 1:
Input: root = [1,2,3,null,5,null,4]
Output: [1,3,4]

Example 2:
Input: root = [1,null,3]
Output: [1,3]

Example 3:
Input: root = []
Output: []

 
Constraints:

	The number of nodes in the tree is in the range [0, 100].
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(node) = root {
            let mut q = std::collections::VecDeque::new();
            q.push_back(node);
            while !q.is_empty() {
                let mut n = q.len();
                let mut got = false;
                while let Some(front) = q.pop_front() {
                    if !got {
                        ans.push(front.borrow().val);
                        got = true;
                    }
                    if let Some(right) = front.borrow_mut().right.take() {
                        q.push_back(right);
                    }
                    if let Some(left) = front.borrow_mut().left.take() {
                        q.push_back(left);
                    }
                    n -= 1;
                    if n == 0 { break; }
                }
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
