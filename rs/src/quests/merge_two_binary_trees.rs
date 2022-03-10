/**
 * [617] Merge Two Binary Trees
 *
 * You are given two binary trees root1 and root2.
Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
Return the merged tree.
Note: The merging process must start from the root nodes of both trees.
 
Example 1:
Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
Output: [3,4,5,5,4,null,7]

Example 2:
Input: root1 = [1], root2 = [1,2]
Output: [2,2]

 
Constraints:

	The number of nodes in both trees is in the range [0, 2000].
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
    pub fn merge_trees(mut root1: Option<Rc<RefCell<TreeNode>>>, mut root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1.take(), root2.take()) {
            (Some(x), Some(y)) => {
                {
                    let mut root = x.borrow_mut();
                    root.val += y.borrow().val;
                    root.left = Self::merge_trees(root.left.take(), y.borrow_mut().left.take());
                    root.right = Self::merge_trees(root.right.take(), y.borrow_mut().right.take());
                }
                Some(x)
            },
            (Some(x), _) | (_, Some(x)) => Some(x),
            _ => None,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(tree![3,4,5,5,4,null,7],
        Solution::merge_trees(tree![1,3,2,5], tree![2,1,3,null,4,null,7])
        );

        assert_eq!(tree![2,2],
        Solution::merge_trees(tree![1], tree![1,2])
        );
    }
}
