/**
 * [206] Reverse Linked List
 *
 * Given the head of a singly linked list, reverse the list, and return the reversed list.
 
Example 1:
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]

Example 2:
Input: head = [1,2]
Output: [2,1]

Example 3:
Input: head = []
Output: []

 
Constraints:

	The number of nodes in the list is the range [0, 5000].
	-5000 <= Node.val <= 5000

 
Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?

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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        // let mut dummy = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            node.next = dummy.next.take();
            dummy.next = Some(node);
        }
        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(linked![1,2,3,4], Solution::reverse_list(linked![4,3,2,1]));
    }
}
