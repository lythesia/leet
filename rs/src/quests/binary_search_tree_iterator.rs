/**
 * [173] Binary Search Tree Iterator
 *
 * Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):

	BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
	boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
	int next() Moves the pointer to the right, then returns the number at the pointer.

Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.
You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.
 
Example 1:
Input
["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
[[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
Output
[null, 3, 7, true, 9, true, 15, true, 20, false]
Explanation
BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
bSTIterator.next();    // return 3
bSTIterator.next();    // return 7
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 9
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 15
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 20
bSTIterator.hasNext(); // return False

 
Constraints:

	The number of nodes in the tree is in the range [1, 105].
	0 <= Node.val <= 106
	At most 105 calls will be made to hasNext, and next.

 
Follow up:

	Could you implement next() and hasNext() to run in average O(1) time and use O(h) memory, where h is the height of the tree?

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
use std::{rc::Rc, cell::RefCell};
type Tree = Rc<RefCell<TreeNode>>;
struct BSTIterator {
    stack: Vec<Tree>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let dummy = Self {
            stack: vec![],
        };
        root.map(|root| Self {
            stack: vec![root],
        }).unwrap_or(dummy)
    }
    
    // if let Some(left) => dfs(left)
    // root
    // if let Some(right) => dfs(right)
    fn next(&mut self) -> i32 {
        if let Some(top) = self.stack.pop() {
            let mut root = top;
            loop {
                let left_node = root.borrow_mut().left.take();
                if let Some(left) = left_node {
                    self.stack.push(root.clone());
                    root = left;
                } else { break; }
            }
            let ans = root.borrow().val;
            if let Some(right) = root.borrow_mut().right.take() {
                self.stack.push(right);
            }
            ans
        } else {
            unreachable!()
        }
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut it = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
        println!("{}", it.next());
        println!("{}", it.next());
        println!("{}", it.has_next());
        println!("{}", it.next());
        println!("{}", it.has_next());
        println!("{}", it.next());
        println!("{}", it.has_next());
        println!("{}", it.next());
        println!("{}", it.has_next());
    }
}
