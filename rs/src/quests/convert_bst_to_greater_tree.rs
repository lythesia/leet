/**
 * [538] Convert BST to Greater Tree
 *
 * Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.
As a reminder, a binary search tree is a tree that satisfies these constraints:

	The left subtree of a node contains only nodes with keys less than the node's key.
	The right subtree of a node contains only nodes with keys greater than the node's key.
	Both the left and right subtrees must also be binary search trees.

 
Example 1:
Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

Example 2:
Input: root = [0,null,1]
Output: [1,null,1]

 
Constraints:

	The number of nodes in the tree is in the range [0, 104].
	-104 <= Node.val <= 104
	All the values in the tree are unique.
	root is guaranteed to be a valid binary search tree.

 
Note: This question is the same as 1038: https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/

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
fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
	if let Some(root) = root {
		let right = root.borrow_mut().right.take();
		let new_right = dfs(right, sum);
		root.borrow_mut().right = new_right;
		*sum += root.borrow().val;
		root.borrow_mut().val = *sum;
		let left = root.borrow_mut().left.take();
		let new_left = dfs(left, sum);
		root.borrow_mut().left = new_left;
		Some(root)
	} else { None }
}
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		let mut sum = 0;
		dfs(root, &mut sum)
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
