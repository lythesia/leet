/**
 * [235] Lowest Common Ancestor of a Binary Search Tree
 *
 * Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.
According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”
 
Example 1:
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
Output: 6
Explanation: The LCA of nodes 2 and 8 is 6.

Example 2:
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
Output: 2
Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.

Example 3:
Input: root = [2,1], p = 2, q = 1
Output: 2

 
Constraints:

	The number of nodes in the tree is in the range [2, 105].
	-109 <= Node.val <= 109
	All Node.val are unique.
	p != q
	p and q will exist in the BST.

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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    fn helper (root: Option<Node>, pval: i32, qval: i32) -> Option<Node> {
        let val = root.as_ref().unwrap().borrow().val;
        if pval > qval {
            Self::helper(root, qval, pval)
        } else if pval <= val && val <= qval {
            root
        } else if val < pval && val < qval {
            Self::helper(root.unwrap().borrow_mut().right.clone(), pval, qval)
        } else {
            Self::helper(root.unwrap().borrow_mut().left.clone(), pval, qval)
        }
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let pval = p.unwrap().borrow().val;
        let qval = q.unwrap().borrow().val;
        Self::helper(root, pval, qval)
    }
    /*
    fn tree_path(root: Option<Node>, val: i32, path: &mut Vec<Node>) {
        if let Some(root) = root {
            let v = root.borrow().val;
            path.push(root.clone());
            if v == val {
                return;
            }
            else if v > val {
                Self::tree_path(root.borrow_mut().left.clone(), val, path)
            } else {
                Self::tree_path(root.borrow_mut().right.clone(), val, path)
            }
        }
    }

    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut path_p = vec![];
        Self::tree_path(root.clone(), p.unwrap().borrow().val, &mut path_p);
        let mut path_q = vec![];
        Self::tree_path(root.clone(), q.unwrap().borrow().val, &mut path_q);
        let mut ans = root;
        let mut it_p = path_p.into_iter();
        let mut it_q = path_q.into_iter();
        loop {
            match (it_p.next(), it_q.next()) {
                (Some(x), Some(y)) => {
                    if x.borrow().val == y.borrow().val {
                        ans = Some(x);
                    } else { break; }
                },
                _ => break,
            }
        }
        ans
    }
    */
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
