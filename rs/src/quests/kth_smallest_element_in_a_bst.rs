/**
 * [230] Kth Smallest Element in a BST
 *
 * Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.
 
Example 1:
Input: root = [3,1,4,null,2], k = 1
Output: 1

Example 2:
Input: root = [5,3,6,2,4,null,null,1], k = 3
Output: 3

 
Constraints:

	The number of nodes in the tree is n.
	1 <= k <= n <= 104
	0 <= Node.val <= 104

 
Follow up: If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?

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
fn in_order(root: Option<Rc<RefCell<TreeNode>>>, k: i32, i: &mut i32, ans: &mut i32) -> bool {
    if let Some(root) = root {
        let left = root.borrow_mut().left.take();
        if in_order(left, k, i, ans) {
            return true;
        }

        *i += 1;
        if *i == k {
            *ans = root.borrow().val;
            return true;
        }

        let right = root.borrow_mut().right.take();
        if in_order(right, k, i, ans) {
            return true;
        }
    }
    false
}
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // let mut ans = -1;
        // let mut i = 0;
        // in_order(root, k, &mut i, &mut ans);
        // ans
        let mut stack = vec![];
        while let Some(mut left) = root.take() {
            root = left.borrow_mut().left.take();
            stack.push(left);
        }
        let mut ans = stack.last().unwrap().borrow().val;
        for _ in 0..k {
            let top = stack.pop().unwrap();
            ans = top.borrow().val;
            let mut right = top.borrow_mut().right.take();
            while let Some(mut n) = right.take() {
                right = n.borrow_mut().left.take();
                stack.push(n);
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::kth_smallest(tree![3,1,4,null,2], 1));
        assert_eq!(3, Solution::kth_smallest(tree![5,3,6,2,4,null,null,1], 3));
    }
}
