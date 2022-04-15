/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
Clarification: The input/output format is the same as how LeetCode serializes a binary tree. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.
 
Example 1:
Input: root = [1,2,3,null,null,4,5]
Output: [1,2,3,null,null,4,5]

Example 2:
Input: root = []
Output: []

 
Constraints:

	The number of nodes in the tree is in the range [0, 104].
	-1000 <= Node.val <= 1000

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
        Self {}
    }

    fn do_ser(root: Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
        if let Some(root) = root {
            s.push_str(&format!("{} ", root.borrow().val));
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            Self::do_ser(left, s);
            Self::do_ser(right, s);
        } else {
            s.push_str("# ");
        }
    }
    fn do_deser(s: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if s.is_empty() { return None; }
        let l = s.remove(0);
        if l == "#" { None }
        else {
            let val = l.parse::<i32>().unwrap();
            let mut root = TreeNode::new(val);
            root.left = Self::do_deser(s);
            root.right = Self::do_deser(s);
            Some(Rc::new(RefCell::new(root)))
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut s = String::new();
        Self::do_ser(root, &mut s);
        s.pop();
        s
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = data.split_ascii_whitespace().collect::<Vec<_>>();
        Self::do_deser(&mut v)
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
        let c = Codec::new();
        let t0 = tree![1,2,3,null,null,4,5];
        let s = c.serialize(t0.clone());
        println!("{}", s);

        let t = c.deserialize(s);
        assert_eq!(t, tree![1,2,3,null,null,4,5]);
    }
}
