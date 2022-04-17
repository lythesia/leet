/**
 * [897] Increasing Order Search Tree
 *
 * Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 
Example 1:
Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]

Example 2:
Input: root = [5,1,7]
Output: [1,null,5,null,7]

 
Constraints:

	The number of nodes in the given tree will be in the range [1, 100].
	0 <= Node.val <= 1000

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

fn in_order(root: Option<Rc<RefCell<TreeNode>>>, tail: &mut Rc<RefCell<TreeNode>>) {
    if let Some(mut root) = root {
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        in_order(left, tail);
        tail.borrow_mut().right = Some(root.clone());
        *tail = root;
        in_order(right, tail);
    }
}
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut tail = dummy.clone();
        in_order(root, &mut tail);
        let right = dummy.borrow_mut().right.take();
        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::increasing_bst(tree![5,3,6,2,4,null,8,1,null,null,null,7,9]));
    }
}
