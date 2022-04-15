/**
 * [669] Trim a Binary Search Tree
 *
 * Given the root of a binary search tree and the lowest and highest boundaries as low and high, trim the tree so that all its elements lies in [low, high]. Trimming the tree should not change the relative structure of the elements that will remain in the tree (i.e., any node's descendant should remain a descendant). It can be proven that there is a unique answer.
Return the root of the trimmed binary search tree. Note that the root may change depending on the given bounds.
 
Example 1:
Input: root = [1,0,2], low = 1, high = 2
Output: [1,null,2]

Example 2:
Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
Output: [3,2,null,1]

 
Constraints:

	The number of nodes in the tree in the range [1, 104].
	0 <= Node.val <= 104
	The value of each node in the tree is unique.
	root is guaranteed to be a valid binary search tree.
	0 <= low <= high <= 104

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
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let val = root.borrow().val;
            if val < low {
                // discard all left
                let right = root.borrow_mut().right.take();
                return Self::trim_bst(right, low, high);
            } else if val <= high {
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                root.borrow_mut().left = Self::trim_bst(left, low, high);
                root.borrow_mut().right = Self::trim_bst(right, low, high);
            } else {
                // discard all right
                let left = root.borrow_mut().left.take();
                return Self::trim_bst(left, low, high);
            }
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
