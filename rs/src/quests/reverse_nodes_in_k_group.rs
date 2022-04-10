/**
 * [25] Reverse Nodes in k-Group
 *
 * Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
You may not alter the values in the list's nodes, only nodes themselves may be changed.
 
Example 1:
Input: head = [1,2,3,4,5], k = 2
Output: [2,1,4,3,5]

Example 2:
Input: head = [1,2,3,4,5], k = 3
Output: [3,2,1,4,5]

 
Constraints:

	The number of nodes in the list is n.
	1 <= k <= n <= 5000
	0 <= Node.val <= 1000

 
Follow-up: Can you solve the problem in O(1) extra memory space?

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
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head.take() {
            head = node.next.take();
            node.next = dummy.next.take();
            dummy.next = Some(node);
        }
        dummy.next
    }
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 { return head; }
        let mut v = vec![];
        loop {
            let mut tail = &mut head;
            let mut kk = k - 1;
            while tail.is_some() && kk > 0 {
                tail = &mut tail.as_mut().unwrap().next;
                kk -= 1;
            }
            if kk > 0 || tail.is_none() { break; }
            let mut tail_next = tail.as_mut().unwrap().next.take();
            std::mem::swap(&mut tail_next, &mut head);
            let rev = Self::reverse_list(tail_next);
            if rev.is_some() {
                v.push(rev);
            } else {
                break;
            }
        }
        v.into_iter().rfold(head, |acc, mut rev| {
            // get tail
            let mut tail = &mut rev;
            while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
            assert!(tail.is_some());
            // concat tail with prev head
            tail.as_mut().map(|n| n.next = acc);
            rev
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::linked_list::disp_list;

    use super::*;

    #[test]
    fn test() {
        disp_list(&Solution::reverse_k_group(linked![1,2,3,4,5], 2));
        disp_list(&Solution::reverse_k_group(linked![1,2,3,4,5], 3));
        disp_list(&Solution::reverse_k_group(linked![1,2], 2));
    }
}
