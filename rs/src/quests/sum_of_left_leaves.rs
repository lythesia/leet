/**
 * [404] Sum of Left Leaves
 *
 * Given the root of a binary tree, return the sum of all left leaves.
 
Example 1:
Input: root = [3,9,20,null,null,15,7]
Output: 24
Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.

Example 2:
Input: root = [1]
Output: 0

 
Constraints:

	The number of nodes in the tree is in the range [1, 1000].
	-1000 <= Node.val <= 1000

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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let sum = if left.is_none() && right.is_none() && is_left {
                node.borrow().val
            } else { 0 };
            sum + Self::dfs(left, true) + Self::dfs(right, false)
        } else { 0 }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // if let Some(node) = root {
        //     let mut left = node.borrow_mut().left.take();
        //     let right = node.borrow_mut().right.take();
        //     let mut leftist = 0;
        //     let mut lsum = 0;
        //     while let Some(l) = left {
        //         if l.borrow().left.is_none() && l.borrow().right.is_none() {
        //             leftist = l.borrow().val;
        //             break;
        //         }
        //         left = l.borrow_mut().left.take();
        //         lsum += Self::sum_of_left_leaves(l.borrow_mut().right.take());
        //     }
        //     leftist + lsum + Self::sum_of_left_leaves(right)
        // } else {
        //     0
        // }
        Self::dfs(root, false)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(24, Solution::sum_of_left_leaves(tree![3,9,20,null,null,15,7]));
        assert_eq!(4, Solution::sum_of_left_leaves(tree![1,2,3,4,5]));
        assert_eq!(5, Solution::sum_of_left_leaves(tree![0,2,4,1,null,3,-1,5,1,null,6,null,8]));
    }
}
