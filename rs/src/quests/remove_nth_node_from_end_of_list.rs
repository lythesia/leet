/**
 * [19] Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the nth node from the end of the list and return its head.
 
Example 1:
Input: head = [1,2,3,4,5], n = 2
Output: [1,2,3,5]

Example 2:
Input: head = [1], n = 1
Output: []

Example 3:
Input: head = [1,2], n = 1
Output: [1]

 
Constraints:

	The number of nodes in the list is sz.
	1 <= sz <= 30
	0 <= Node.val <= 100
	1 <= n <= sz

 
Follow up: Could you do this in one pass?

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{val: -1, next: head});
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();
        while n > 0 {
            if fast.next.is_none() { break; }
            fast = fast.next.unwrap();
            n -= 1;
        }
        if n > 0 { return dummy.next; }

        while let Some(fast_next) = fast.next {
            fast = fast_next;
            slow = slow.next.as_mut().unwrap();
        }
        // remove slow->next
        let mut slow_next = slow.next.as_mut().unwrap();
        slow.next = slow_next.next.clone();
        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::linked_list::disp_list;

    use super::*;

    #[test]
    fn test() {
        disp_list(&Solution::remove_nth_from_end(linked![1,2,3,4,5], 2));
        disp_list(&Solution::remove_nth_from_end(linked![1], 1));
        disp_list(&Solution::remove_nth_from_end(linked![1,2], 1));
    }
}
