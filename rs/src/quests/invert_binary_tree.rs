/**
 * [226] Invert Binary Tree
 *
 * Given the root of a binary tree, invert the tree, and return its root.
 
Example 1:
Input: root = [4,2,7,1,3,6,9]
Output: [4,7,2,9,6,3,1]

Example 2:
Input: root = [2,1,3]
Output: [2,3,1]

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let left = Self::invert_tree(root.borrow_mut().right.take());
            let right = Self::invert_tree(root.borrow_mut().left.take());
            root.borrow_mut().left = left;
            root.borrow_mut().right = right;
            Some(root)
        } else { None }
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
