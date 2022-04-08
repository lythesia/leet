/**
 * [143] Reorder List
 *
 * You are given the head of a singly linked-list. The list can be represented as:
L0 → L1 → … → Ln - 1 → Ln

Reorder the list to be on the following form:
L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …

You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 
Example 1:
Input: head = [1,2,3,4]
Output: [1,4,2,3]

Example 2:
Input: head = [1,2,3,4,5]
Output: [1,5,2,4,3]

 
Constraints:

	The number of nodes in the list is in the range [1, 5 * 104].
	1 <= Node.val <= 1000

 */
pub struct Solution {}
use crate::util::linked_list::disp_list;

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if !head.as_ref().map_or(false, |n| n.next.is_some()) { return; }
        // find middle and stack right half
        let mut fast = head.clone();
        let mut slow = head.as_mut();
        while let Some(mut f) = fast.take() {
            if let Some(mut fnext) = f.next.take() {
                fast = fnext.next.take();
                slow = slow.unwrap().next.as_mut();
            } else { break; }
        }
        // move slow ahead coz right half may have one more node
        let mut slow = slow.unwrap().next.take();
        let mut stack = vec![];
        while let Some(mut s) = slow.take() {
            slow = s.next.take();
            stack.push(s);
        }
        let mut curr = head.as_mut();
        while let Some(mut right) = stack.pop() {
            if let Some(c) = curr.take() {
                let mut next = c.next.take();
                right.next = next;
                c.next = Some(right);
                curr = c.next.as_mut().unwrap().next.as_mut();
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::linked_list::disp_list;

    use super::*;

    #[test]
    fn test() {
        let mut l = linked![1,2,3,4,5];
        Solution::reorder_list(&mut l);
        disp_list(&l);

        let mut l = linked![1,2,3,4];
        Solution::reorder_list(&mut l);
        disp_list(&l);
    }
}
