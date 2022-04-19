/**
 * [99] Recover Binary Search Tree
 *
 * You are given the root of a binary search tree (BST), where the values of exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.
 
Example 1:
Input: root = [1,3,null,null,2]
Output: [3,1,null,null,2]
Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.

Example 2:
Input: root = [3,1,4,null,null,2]
Output: [2,1,4,null,null,3]
Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.

 
Constraints:

	The number of nodes in the tree is in the range [2, 1000].
	-231 <= Node.val <= 231 - 1

 
Follow up: A solution using O(n) space is pretty straight-forward. Could you devise a constant O(1) space solution?
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
fn in_order(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Rc<RefCell<TreeNode>>>) {
    if let Some(root) = root {
        let left = root.borrow_mut().left.clone();
        let right = root.borrow_mut().right.clone();
        in_order(left, v);
        v.push(root);
        in_order(right, v);
    }
}
fn in_order_swap(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>,
    fst: &mut Option<Rc<RefCell<TreeNode>>>, snd: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root) = root {
        let left = root.borrow_mut().left.clone();
        let right = root.borrow_mut().right.clone();
        in_order_swap(left, prev, fst, snd);
        if let Some(prev) = prev {
            if prev.borrow().val > root.borrow().val {
                if fst.is_none() { *fst = Some(prev.clone()); }
                if fst.is_some() { *snd = Some(root.clone()); }
            }
        }
        *prev = Some(root);
        in_order_swap(right, prev, fst, snd);
    }
}
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // let mut v = vec![];
        // in_order(root.clone(), &mut v);
        // let mut prev = v.remove(0);
        // let mut fst = vec![];
        // let mut snd = vec![];
        // for x in v {
        //     if x.borrow().val < prev.borrow().val {
        //         if fst.is_empty() {
        //             fst.extend([prev.clone(), x.clone()]);
        //         } else {
        //             snd.extend([prev.clone(), x.clone()]);
        //             break;
        //         }
        //     }
        //     prev = x;
        // }
        // if snd.is_empty() {
        //     snd = fst.clone();
        // }
        // use std::cmp::{min_by_key,max_by_key};
        // let fst = max_by_key(fst.pop().unwrap(), fst.pop().unwrap(), |x| x.borrow().val);
        // let snd = min_by_key(snd.pop().unwrap(), snd.pop().unwrap(), |x| x.borrow().val);
        // std::mem::swap(&mut fst.borrow_mut().val, &mut snd.borrow_mut().val);
        let (mut prev, mut fst, mut snd) = (None, None, None);
        in_order_swap(root.clone(), &mut prev, &mut fst, &mut snd);
        let fst = fst.unwrap();
        let snd = snd.unwrap();
        std::mem::swap(&mut fst.borrow_mut().val, &mut snd.borrow_mut().val);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut root = tree![3,1,4,null,null,2]; // 1 3 2 4
        Solution::recover_tree(&mut root);
    }
}
