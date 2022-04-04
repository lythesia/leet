/**
 * [1721] Swapping Nodes in a Linked List
 *
 * You are given the head of a linked list, and an integer k.
Return the head of the linked list after swapping the values of the kth node from the beginning and the kth node from the end (the list is 1-indexed).
 
Example 1:
Input: head = [1,2,3,4,5], k = 2
Output: [1,4,3,2,5]

Example 2:
Input: head = [7,9,6,6,7,8,3,0,9,5], k = 5
Output: [7,9,6,6,8,7,3,0,9,5]

 
Constraints:

	The number of nodes in the list is n.
	1 <= k <= n <= 105
	0 <= Node.val <= 100

 */
pub struct Solution {}
use super::super::util::linked_list::{ListNode, to_list};

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
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = vec![];
        while let Some(mut node) = head.take() {
            head = node.next.take();
            v.push(node);
        }
        let n = v.len();
        let k = k as usize;
        v.swap(k-1, n-k);
        let mut current = None;
        for mut node in v.into_iter().rev() {
            node.next = current;
            current = Some(node);
        }
        current
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
