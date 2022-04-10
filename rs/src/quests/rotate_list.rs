/**
 * [61] Rotate List
 *
 * Given the head of a linked list, rotate the list to the right by k places.
 
Example 1:
Input: head = [1,2,3,4,5], k = 2
Output: [4,5,1,2,3]

Example 2:
Input: head = [0,1,2], k = 4
Output: [2,0,1]

 
Constraints:

	The number of nodes in the list is in the range [0, 500].
	-100 <= Node.val <= 100
	0 <= k <= 2 * 109

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }
        let mut len = 0;
        // len
        let mut p = head.as_ref();
        while let Some(node) = p.take() {
            p = node.next.as_ref();
            len += 1;
        }
        let mut k = k % len;
        if k < 1 { return head; }

        // pk: move ahead: [n - k - 1] pk [k]
        let mut pk = &mut head;
        for _ in 0..len-k-1 {
            pk = &mut pk.as_mut().unwrap().next;
        }
        assert!(pk.is_some());
        // [n - k - 1] pk /cut/ new_head [k]
        let mut new_head = pk.as_mut().unwrap().next.take();
        let mut tail = &mut new_head;
        while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        assert!(tail.is_some());
        tail.as_mut().map(|n| n.next = head);
        new_head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::linked_list::disp_list;

    use super::*;

    #[test]
    fn test() {
        disp_list(&Solution::rotate_right(linked![1,2,3,4,5], 2));
    }
}
