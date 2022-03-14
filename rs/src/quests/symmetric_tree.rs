/**
 * [101] Symmetric Tree
 *
 * Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
 
Example 1:
Input: root = [1,2,2,3,4,4,3]
Output: true

Example 2:
Input: root = [1,2,2,null,3,null,3]
Output: false

 
Constraints:

	The number of nodes in the tree is in the range [1, 1000].
	-100 <= Node.val <= 100

 
Follow up: Could you solve it both recursively and iteratively?
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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn dfs(x: Node, y: Node) -> bool {
        match (x, y) {
            (Some(l), Some(r)) => l.borrow().val == r.borrow().val
                && Self::dfs(l.borrow().left.clone(), r.borrow().right.clone())
                && Self::dfs(l.borrow().right.clone(), r.borrow().left.clone()),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut r = root.borrow_mut();
            Self::dfs(r.left.take(), r.right.take())
        } else {
            true
        }
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
