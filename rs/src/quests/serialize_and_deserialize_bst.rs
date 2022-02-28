/**
 * [449] Serialize and Deserialize BST
 *
 * Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
Design an algorithm to serialize and deserialize a binary search tree. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.
The encoded string should be as compact as possible.
 
Example 1:
Input: root = [2,1,3]
Output: [2,1,3]
Example 2:
Input: root = []
Output: []
 
Constraints:

	The number of nodes in the tree is in the range [0, 104].
	0 <= Node.val <= 104
	The input tree is guaranteed to be a binary search tree.

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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(n) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(n);
            let mut ans = String::new();
            while let Some(top) = queue.pop_front() {
                if !ans.is_empty() {
                    ans.push('#');
                }
                let mut r = top.borrow_mut();
                let v = r.val.to_string();
                ans += &v;
                if let Some(left) = r.left.take() { queue.push_back(left); }
                if let Some(right) = r.right.take() { queue.push_back(right); }
            }
            ans
        }
        else {
            "".into()
        }
    }

    fn bst_insert(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        let new = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        if let Some(r) = root {
            let mut r = r.borrow_mut();
            if val < r.val {
                Self::bst_insert(&mut r.left, val);
            } else {
                Self::bst_insert(&mut r.right, val);
            }
        } else {
            *root = new;
        }
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        for val in data.split('#') {
            Self::bst_insert(&mut root, val.parse().unwrap());
        }
        root
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
