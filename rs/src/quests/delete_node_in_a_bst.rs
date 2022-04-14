/**
 * [450] Delete Node in a BST
 *
 * Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
Basically, the deletion can be divided into two stages:

	Search for a node to remove.
	If the node is found, delete the node.

 
Example 1:
Input: root = [5,3,6,2,4,null,7], key = 3
Output: [5,4,6,2,null,null,7]
Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.

Example 2:
Input: root = [5,3,6,2,4,null,7], key = 0
Output: [5,3,6,2,4,null,7]
Explanation: The tree does not contain a node with value = 0.

Example 3:
Input: root = [], key = 0
Output: []

 
Constraints:

	The number of nodes in the tree is in the range [0, 104].
	-105 <= Node.val <= 105
	Each node has a unique value.
	root is a valid binary search tree.
	-105 <= key <= 105

 
Follow up: Could you solve it with time complexity O(height of tree)?

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
	fn delete(root: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
		if let Some(root) = root {
			let val = root.borrow().val;
			if key < val {
				let l = Self::delete(&root.borrow().left, key);
				root.borrow_mut().left = l;
			} else if key > val {
				let r =  Self::delete(&root.borrow().right, key);
				root.borrow_mut().right = r;
			} else {
				if root.borrow().left.is_none() {
					return root.borrow_mut().right.take();
				}
				if root.borrow().right.is_none() {
					return root.borrow_mut().left.take();
				}
				assert!(root.borrow().right.is_some());
				// right has left?
				let mut leftist = root.borrow().right.clone();
				while let Some(n) = leftist.clone() {
					if n.borrow().left.is_some() {
						leftist = n.borrow().left.clone();
					} else {
						// leftist = Some(n); // restore if we use take in match
						break;
					}
				}
				if let Some(n) = leftist {
					let v = n.borrow().val;
					let r = Solution::delete(&root.borrow().right, v);
					root.borrow_mut().val = v;
					root.borrow_mut().right = r;
				}
			}
		}
		root.clone()
	}

    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
		Self::delete(&root, key)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert_eq!(tree![5,4,6,2,null,null,7], Solution::delete_node(tree![5,3,6,2,4,null,7], 3));
    }
}
