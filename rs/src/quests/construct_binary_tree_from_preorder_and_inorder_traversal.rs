/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
 
Example 1:
Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
Output: [3,9,20,null,null,15,7]

Example 2:
Input: preorder = [-1], inorder = [-1]
Output: [-1]

 
Constraints:

	1 <= preorder.length <= 3000
	inorder.length == preorder.length
	-3000 <= preorder[i], inorder[i] <= 3000
	preorder and inorder consist of unique values.
	Each value of inorder also appears in preorder.
	preorder is guaranteed to be the preorder traversal of the tree.
	inorder is guaranteed to be the inorder traversal of the tree.

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		if preorder.is_empty() { return None; }

		assert!(preorder.len() == inorder.len());
		let root_val = preorder[0];
		let lcnt = inorder.iter().position(|&x| x == root_val).unwrap();
		let left = Self::build_tree(preorder[1..1+lcnt].to_vec(), inorder[0..lcnt].to_vec());
		let right = Self::build_tree(preorder[1+lcnt..].to_vec(), inorder[lcnt+1..].to_vec());
		Some(Rc::new(RefCell::new(TreeNode { val: root_val, left, right })))
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
