/**
 * [112] Path Sum
 *
 * Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
A leaf is a node with no children.
 
Example 1:
Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
Output: true
Explanation: The root-to-leaf path with the target sum is shown.

Example 2:
Input: root = [1,2,3], targetSum = 5
Output: false
Explanation: There two root-to-leaf paths in the tree:
(1 --> 2): The sum is 3.
(1 --> 3): The sum is 4.
There is no root-to-leaf path with sum = 5.

Example 3:
Input: root = [], targetSum = 0
Output: false
Explanation: Since the tree is empty, there are no root-to-leaf paths.

 
Constraints:

	The number of nodes in the tree is in the range [0, 5000].
	-1000 <= Node.val <= 1000
	-1000 <= targetSum <= 1000

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let r = root.borrow();
            let (left, right) = (r.left.clone(), r.right.clone());
            if left.is_none() && right.is_none() {
                target_sum == r.val
            } else {
                Self::has_path_sum(left, target_sum - r.val) || Self::has_path_sum(right, target_sum - r.val)
            }
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::has_path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,null,1], 22));
        assert!(!Solution::has_path_sum(tree![1,2,3], 5));
        assert!(Solution::has_path_sum(tree![], 0));
    }
}
