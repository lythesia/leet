/**
 * [113] Path Sum II
 *
 * Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where the sum of the node values in the path equals targetSum. Each path should be returned as a list of the node values, not node references.
A root-to-leaf path is a path starting from the root and ending at any leaf node. A leaf is a node with no children.
 
Example 1:
Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
Output: [[5,4,11,2],[5,8,4,5]]
Explanation: There are two paths whose sum equals targetSum:
5 + 4 + 11 + 2 = 22
5 + 8 + 4 + 5 = 22

Example 2:
Input: root = [1,2,3], targetSum = 5
Output: []

Example 3:
Input: root = [1,2], targetSum = 0
Output: []

 
Constraints:

	The number of nodes in the tree is in the range [0, 5000].
	-1000 <= Node.val <= 1000
	-1000 <= targetSum <= 1000

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
fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, t: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        let val = node.borrow().val;
        path.push(val);
        match (left.is_some(), right.is_some()) {
            (false, false) => {
                if sum + val == t {
                    ans.push(path.clone());
                }
            },
            (true, false) => {
                dfs(left, sum + val, t, path, ans);
            },
            (false, true) => {
                dfs(right, sum + val, t, path, ans);
            },
            _ => {
                dfs(left, sum + val, t, path, ans);
                dfs(right, sum + val, t, path, ans);
            },
        }
        path.pop();
    }
}
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];
        dfs(root, 0, target_sum, &mut path, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,5,1], 22));
    }
}
