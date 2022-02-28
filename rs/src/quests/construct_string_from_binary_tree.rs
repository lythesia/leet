/**
 * [606] Construct String from Binary Tree
 *
 * Given the root of a binary tree, construct a string consisting of parenthesis and integers from a binary tree with the preorder traversal way, and return it.
Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship between the string and the original binary tree.
 
Example 1:
Input: root = [1,2,3,4]
Output: "1(2(4))(3)"
Explanation: Originally, it needs to be "1(2(4)())(3()())", but you need to omit all the unnecessary empty parenthesis pairs. And it will be "1(2(4))(3)"

Example 2:
Input: root = [1,2,3,null,4]
Output: "1(2()(4))(3)"
Explanation: Almost the same as the first example, except we cannot omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.

 
Constraints:

	The number of nodes in the tree is in the range [1, 104].
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
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        /*
        if let Some(root) = root {
            let r = root.borrow();
            let mut s = r.val.to_string();
            let ls = Self::tree2str(r.left.clone());
            let rs = Self::tree2str(r.right.clone());
            // _ #
            if !rs.is_empty() {
                s.push('(');
                s.push_str(&ls);
                s.push_str(")(");
                s.push_str(&rs);
                s.push(')');
            }
            // # ""
            else if !ls.is_empty() {
                s.push('(');
                s.push_str(&ls);
                s.push(')');
            }
            // else "" ""
            s
        } else {
            "".into()
        }
        */
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
            if let Some(root) = root {
                let r = root.borrow();
                *s += &r.val.to_string();
                if r.left.is_some() || r.right.is_some() {
                    s.push('(');
                    helper(r.left.clone(), s);
                    s.push(')');

                    if r.right.is_some() {
                        s.push('(');
                        helper(r.right.clone(), s);
                        s.push(')');
                    }
                }
            }
        }
        let mut ans = String::new();
        helper(root, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("1(2(4))(3)", Solution::tree2str(tree![1,2,3,4]));
        assert_eq!("1(2()(4))(3)", Solution::tree2str(tree![1,2,3,null,4]));
    }
}
