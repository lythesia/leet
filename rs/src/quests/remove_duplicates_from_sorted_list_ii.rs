/**
 * [82] Remove Duplicates from Sorted List II
 *
 * Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
 
Example 1:
Input: head = [1,2,3,3,4,4,5]
Output: [1,2,5]

Example 2:
Input: head = [1,1,1,2,3]
Output: [2,3]

 
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
        if head.is_none() { return head; }

        let mut last = 101;
        let mut dummy = ListNode { val: last, next: head };
        let mut node = &mut dummy.next;
        loop {
            match node {
                // delete
                Some(n) if n.val == last => *node = n.next.take(),
                // found consecutive, update last and repeat curr node once more
                Some(n) if n.next.is_some() && n.next.as_ref().unwrap().val == n.val => last = n.val,
                // else curr node != last && no consecutive
                // set proper curr node and last, and repeat this node once more
                Some(n) => {
                    node = &mut n.next;
                    if let Some(n) = node {
                        last = n.val - 1;
                    }
                },
                _ => return dummy.next,
            }
        }
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
