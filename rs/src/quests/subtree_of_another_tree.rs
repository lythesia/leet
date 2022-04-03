/**
 * [572] Subtree of Another Tree
 *
 * Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.
 
Example 1:
Input: root = [3,4,5,1,2], subRoot = [4,1,2]
Output: true

Example 2:
Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
Output: false

 
Constraints:

	The number of nodes in the root tree is in the range [1, 2000].
	The number of nodes in the subRoot tree is in the range [1, 1000].
	-104 <= root.val <= 104
	-104 <= subRoot.val <= 104

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
type NodePtr = Option<Rc<RefCell<TreeNode>>>;
fn is_same(x: NodePtr, y: NodePtr) -> bool {
    match (x, y) {
        (Some(x), Some(y)) if x.borrow().val == y.borrow().val => {
            let lx = x.borrow_mut().left.clone();
            let rx = x.borrow_mut().right.clone();
            let ly = y.borrow_mut().left.clone();
            let ry = y.borrow_mut().right.clone();
            is_same(lx, ly) && is_same(rx, ry)
        },
        (None, None) => true,
        _ => false,
    }
}
impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if is_same(root.clone(), sub_root.clone()) { return true; }
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            Self::is_subtree(left, sub_root.clone()) || Self::is_subtree(right, sub_root)
        } else { false }
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
