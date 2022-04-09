/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
 
Example 1:
Input: head = [1,2,3,4]
Output: [2,1,4,3]

Example 2:
Input: head = []
Output: []

Example 3:
Input: head = [1]
Output: [1]

 
Constraints:

	The number of nodes in the list is in the range [0, 100].
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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // if head.is_none() { return None; }

        // let mut curr = head;
        // let mut new_head = None;
        // if let Some(mut curr_node) = curr.take() {
        //     let next = curr_node.next.take();
        //     if let Some(mut next_node) = next {
        //         curr_node.next = Solution::swap_pairs(next_node.next.take());
        //         next_node.next = Some(curr_node);
        //         new_head = Some(next_node);
        //     } else {
        //         new_head = Some(curr_node);
        //     }
        // }
        // new_head
        head.and_then(|mut curr| {
            match curr.next {
                Some(mut next) => {
                    curr.next = Solution::swap_pairs(next.next.take());
                    next.next = Some(curr);
                    Some(next)
                },
                _ => Some(curr)
            }
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
        disp_list(&Solution::swap_pairs(linked!(1,2,3,4)));
    }
}
