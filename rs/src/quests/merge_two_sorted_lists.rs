/**
 * [21] Merge Two Sorted Lists
 *
 * You are given the heads of two sorted linked lists list1 and list2.
Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
Return the head of the merged linked list.
 
Example 1:
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:
Input: list1 = [], list2 = []
Output: []

Example 3:
Input: list1 = [], list2 = [0]
Output: [0]

 
Constraints:

	The number of nodes in both lists is in the range [0, 50].
	-100 <= Node.val <= 100
	Both list1 and list2 are sorted in non-decreasing order.

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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        loop {
            match (list1.take(), list2.take()) {
                (Some(mut x), Some(mut y)) => {
                    let mut next = if x.val < y.val {
                        list1 = x.next.take();
                        list2 = Some(y);
                        x
                    } else {
                        list1 = Some(x);
                        list2 = y.next.take();
                        y
                    };
                    print!("{} -> ", next.val);
                    tail.as_mut().unwrap().next = Some(next); // tail.next = next;
                    tail = &mut tail.as_mut().unwrap().next; // tail = tail.next;
                },
                (Some(mut next), _) | (_, Some(mut next)) => {
                    print!("{} -> ", next.val);
                    tail.as_mut().unwrap().next = Some(next); // tail.next = next;
                    break;
                },
                _ => break,
            }
        }
        println!();
        head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Solution::merge_two_lists(linked![1,2,4], linked![1,3,4]);
    }
}
