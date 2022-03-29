/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
For this problem, a height-balanced binary tree is defined as:

a binary tree in which the left and right subtrees of every node differ in height by no more than 1.

 
Example 1:
Input: root = [3,9,20,null,null,15,7]
Output: true

Example 2:
Input: root = [1,2,2,3,3,null,null,4,4]
Output: false

Example 3:
Input: root = []
Output: true

 
Constraints:

	The number of nodes in the tree is in the range [0, 5000].
	-104 <= Node.val <= 104

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut r = root.borrow_mut();
            let left = r.left.take();
            let right = r.right.take();
            if Self::is_balanced(left.clone()) && Self::is_balanced(right.clone()) {
                let hl = left.map(|x| x.borrow().val).unwrap_or(0);
                let hr = right.map(|x| x.borrow().val).unwrap_or(0);
                r.val = 1 + std::cmp::max(hl, hr);
                (hl - hr).abs() <= 1
            } else { false }
        } else { true }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_balanced(tree![3,9,20,null,null,15,7]));
        assert!(!Solution::is_balanced(tree![1,2,2,3,3,null,null,4,4]));
    }
}
