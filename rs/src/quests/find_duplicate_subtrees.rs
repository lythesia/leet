/**
 * [652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
For each kind of duplicate subtrees, you only need to return the root node of any one of them.
Two trees are duplicate if they have the same structure with the same node values.
 
Example 1:
Input: root = [1,2,3,4,null,2,4,null,null,4]
Output: [[2,4],[4]]

Example 2:
Input: root = [2,1,1]
Output: [[1]]

Example 3:
Input: root = [2,2,2,3,null,3,null]
Output: [[2,3],[3]]

 
Constraints:

	The number of the nodes in the tree will be in the range [1, 10^4]
	-200 <= Node.val <= 200

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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    fn helper(root: Option<Node>, h: &mut HashMap<String, i32>, v: &mut Vec<Option<Node>>) -> String {
        if let Some(root) = root {
            let r = root.borrow();
            let s = r.val.to_string() + "#" +
                &Self::helper(r.left.clone(), h, v) + "#" +
                &Self::helper(r.right.clone(), h, v);
            // why in-fix order fail? because in-fix desered string is symmetric!
            // so only pre/post order succeed
            // let mut s = String::new();
            // s.push_str(&Self::helper(r.left.clone(), h, v));
            // s.push('#');
            // s.push_str(&r.val.to_string());
            // s.push('#');
            // s.push_str(&Self::helper(r.right.clone(), h, v));
            let c = h.entry(s.clone()).or_insert(0);
            if *c == 1 { v.push(Some(root.clone())); }
            *c += 1;
            s
        } else {
            "".into()
        }
    }

    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![];
        let mut h = std::collections::HashMap::new();
        Self::helper(root, &mut h, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![tree!(0)],
            Solution::find_duplicate_subtrees(tree!(0,0,0,0,null,null,0,null,null,null,0)));
    }
}
