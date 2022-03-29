/**
 * [1367] Linked List in Binary Tree
 *
 * Given a binary tree root and a linked list with head as the first node. 
Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
In this context downward path means a path that starts at some node and goes downwards.
 
Example 1:

Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: true
Explanation: Nodes in blue form a subpath in the binary Tree.  

Example 2:

Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: true

Example 3:
Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: false
Explanation: There is no path in the binary tree that contains all the elements of the linked list from head.

 
Constraints:

	The number of nodes in the tree will be in the range [1, 2500].
	The number of nodes in the list will be in the range [1, 100].
	1 <= Node.val <= 100 for each node in the linked list and binary tree.

 */
pub struct Solution {}
use super::super::util::linked_list::{ListNode, to_list};
use super::super::util::tree::{TreeNode, to_tree};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    fn is_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>, start: bool) -> bool {
        match (head, root) {
            (Some(head), Some(root)) => {
                let left = root.borrow().left.clone();
                let right = root.borrow().right.clone();
                if start {
                    head.val == root.borrow().val &&
                    (
                        Self::is_path(head.next.clone(), left, start) ||
                        Self::is_path(head.next.clone(), right, start)
                    )
                } else {
                    if head.val == root.borrow().val {
                        if Self::is_path(head.next.clone(), left.clone(), true) ||
                            Self::is_path(head.next.clone(), right.clone(), true) {
                            return true;
                        }
                    }
                    Self::is_path(Some(head.clone()), left, start) ||
                    Self::is_path(Some(head), right, start)
                }
            },
            (Some(_), _) => false,
            (_, _) => true,
        }
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_path(head, root, false)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_sub_path(linked![4,2,8], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]));
        assert!(Solution::is_sub_path(linked![1,4,2,6], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]));
        assert!(!Solution::is_sub_path(linked![1,4,2,6,8], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]));
    }
}
