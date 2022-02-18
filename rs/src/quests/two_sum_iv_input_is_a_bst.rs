/**
 * [653] Two Sum IV - Input is a BST
 *
 * Given the root of a Binary Search Tree and a target number k, return true if there exist two elements in the BST such that their sum is equal to the given target.

Example 1:
Input: root = [5,3,6,2,4,null,7], k = 9
Output: true

Example 2:
Input: root = [5,3,6,2,4,null,7], k = 28
Output: false

Example 3:
Input: root = [2,1,3], k = 4
Output: true

Example 4:
Input: root = [2,1,3], k = 1
Output: false

Example 5:
Input: root = [2,1,3], k = 3
Output: true


Constraints:

	The number of nodes in the tree is in the range [1, 104].
	-104 <= Node.val <= 104
	root is guaranteed to be a valid binary search tree.
	-105 <= k <= 105

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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        dummy.borrow_mut().left = root.clone();
        dummy.borrow_mut().right = root.clone();
        let (mut left_stack, mut right_stack) = (vec![dummy.clone()], vec![dummy]);
        Self::preorder_next(&mut left_stack);
        Self::postorder_next(&mut right_stack);

        while !left_stack.is_empty() && !right_stack.is_empty() {
            let (l, r) = (left_stack.last().unwrap(), right_stack.last().unwrap());
            if l.as_ptr() == r.as_ptr() {
                break;
            }

            let (lv, rv) = (l.borrow().val, r.borrow().val);
            let sum = lv + rv;
            println!("match {} + {}", lv, rv);
            if sum == k { return true; }
            else if sum < k { Self::preorder_next(&mut left_stack); }
            else { Self::postorder_next(&mut right_stack); }
        }

        false
    }

    // stack.pop -> current node
    // so next must be current.right
    // if current has left sub-tree, save them to stack
    fn preorder_next(stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let mut root = stack.pop().unwrap().borrow().right.clone();
        while let Some(node) = root {
            stack.push(node.clone());
            root = node.borrow().left.clone();
        }
    }

    fn postorder_next(stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let mut root = stack.pop().unwrap().borrow().left.clone();
        while let Some(node) = root {
            stack.push(node.clone());
            root = node.borrow().right.clone();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(false, Solution::find_target(tree!(5,3,6,2,4,null,7), 28));
        assert_eq!(true, Solution::find_target(tree!(2,1,3), 4));
        assert_eq!(false, Solution::find_target(tree!(2,1,3), 1));
        assert_eq!(true, Solution::find_target(tree!(2,1,3), 3));
        assert_eq!(false, Solution::find_target(tree!(1), 1));
        assert_eq!(true, Solution::find_target(tree![0,-1,2,-3,null,null,4], 4));
        //        0
        //      -1  2
        //    -3 x  x 4
    }
}
