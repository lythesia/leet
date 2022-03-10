/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
 
Example 1:
Input: head = [1,1,2]
Output: [1,2]

Example 2:
Input: head = [1,1,2,3,3]
Output: [1,2,3]

 
Constraints:

	The number of nodes in the list is in the range [0, 300].
	-100 <= Node.val <= 100
	The list is guaranteed to be sorted in ascending order.

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut tail = &mut dummy;
        let mut prev = 101;
        while let Some(mut node) = head {
            head = node.next.take();
            let val = node.val;
            if val != prev {
                tail = &mut tail.insert(node).next;
            }
            prev = val;
        }
        dummy
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(linked![1,2,3], Solution::delete_duplicates(linked![1,1,2,2,3,3]));
    }
}
