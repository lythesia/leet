/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.


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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;
        let (mut l1, mut l2) = (l1, l2);
        let (mut e1, mut e2) = (false, false);
        let mut c = 0;
        loop {
            let x = match l1 {
                Some(n) => { l1 = n.next; n.val },
                _ => { e1 = true; 0 }
            };
            let y = match l2 {
                Some(n) => { l2 = n.next; n.val },
                _ => { e2 = true; 0 }
            };
            if (e1 && e2 && c == 0) {
                break dummy.unwrap().next;
            }
            let s = x + y + c;
            c = s / 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(s % 10)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])), to_list(vec![7, 0, 8]));
        assert_eq!(Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])), to_list(vec![8, 9, 9, 9, 0, 0, 1]));
        assert_eq!(Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])), to_list(vec![0]))
    }
}
