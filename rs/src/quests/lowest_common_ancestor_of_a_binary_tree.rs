/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”
 
Example 1:
Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
Output: 3
Explanation: The LCA of nodes 5 and 1 is 3.

Example 2:
Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
Output: 5
Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.

Example 3:
Input: root = [1,2], p = 1, q = 2
Output: 1

 
Constraints:

	The number of nodes in the tree is in the range [2, 105].
	-109 <= Node.val <= 109
	All Node.val are unique.
	p != q
	p and q will exist in the tree.

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
use std::collections::HashMap;
fn dfs(root: Rc<RefCell<TreeNode>>, fa: &mut HashMap<i32, Rc<RefCell<TreeNode>>>) {
    let left = root.borrow().left.clone();
    left.map(|x| {
        fa.insert(x.borrow().val, root.clone());
        dfs(x, fa);
    });
    let right = root.borrow().right.clone();
    right.map(|x| {
        fa.insert(x.borrow().val, root.clone());
        dfs(x, fa);
    });
}
impl Solution {
    // store fathers
    // pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     let root = root.unwrap();
    //     let mut fa = HashMap::new();
    //     dfs(root.clone(), &mut fa);

    //     let mut vis = HashMap::new();
    //     let mut p = p.as_ref();
    //     while let Some(n) = p.take() {
    //         let val = n.borrow().val;
    //         vis.insert(val, true);
    //         p = fa.get(&val);
    //     }
    //     let mut q = q.as_ref();
    //     while let Some(n) = q.take() {
    //         let val = n.borrow().val;
    //         if vis.contains_key(&val) {
    //             return Some(n.clone());
    //         }
    //         q = fa.get(&val);
    //     }
    //     Some(root)
    // }

    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let p = p.unwrap();
            let q = q.unwrap();
            if Rc::ptr_eq(&root, &p) || Rc::ptr_eq(&root, &q) { Some(root) }
            else {
                let l = Self::lowest_common_ancestor(root.borrow().left.clone(), Some(p.clone()), Some(q.clone()));
                let r = Self::lowest_common_ancestor(root.borrow().right.clone(), Some(p.clone()), Some(q.clone()));
                match (l, r) {
                    (Some(_), Some(_)) => Some(root),
                    (Some(x), _) | (_, Some(x)) => Some(x),
                    _ => None,
                }
            }
        } else { None }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // println!("{:?}", Solution::lowest_common_ancestor(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5], tree![1]));
        println!("{:?}", Solution::lowest_common_ancestor(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5], tree![4]));
    }
}
