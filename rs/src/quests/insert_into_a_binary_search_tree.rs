/**
 * [701] Insert into a Binary Search Tree
 *
 * You are given the root node of a binary search tree (BST) and a value to insert into the tree. Return the root node of the BST after the insertion. It is guaranteed that the new value does not exist in the original BST.
Notice that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion. You can return any of them.
 
Example 1:
Input: root = [4,2,7,1,3], val = 5
Output: [4,2,7,1,3,5]
Explanation: Another accepted tree is:

Example 2:
Input: root = [40,20,60,10,30,50,70], val = 25
Output: [40,20,60,10,30,50,70,null,null,25]

Example 3:
Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
Output: [4,2,7,1,3,5]

 
Constraints:

	The number of nodes in the tree will be in the range [0, 104].
	-108 <= Node.val <= 108
	All the values Node.val are unique.
	-108 <= val <= 108
	It's guaranteed that val does not exist in the original BST.

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
    fn insert(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(root) = root {
            let mut v = root.borrow().val;
            if val < v {
                Self::insert(&mut root.borrow_mut().left, val)
            } else {
                Self::insert(&mut root.borrow_mut().right, val)
            }
        } else {
            root.insert(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }

    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // Self::insert(&mut root, val);
        // root
        if let Some(root) = root {
            if val < root.borrow().val {
                let left = root.borrow_mut().left.take();
                root.borrow_mut().left = Self::insert_into_bst(left, val);
            } else {
                let right = root.borrow_mut().right.take();
                root.borrow_mut().right = Self::insert_into_bst(right, val);
            }
            Some(root)
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(tree![4,2,7,1,3,5], Solution::insert_into_bst(tree![4,2,7,1,3], 5));
    }
}
