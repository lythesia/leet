/**
 * [445] Add Two Numbers II
 *
 * You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 
Example 1:
Input: l1 = [7,2,4,3], l2 = [5,6,4]
Output: [7,8,0,7]

Example 2:
Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [8,0,7]

Example 3:
Input: l1 = [0], l2 = [0]
Output: [0]

 
Constraints:

	The number of nodes in each linked list is in the range [1, 100].
	0 <= Node.val <= 9
	It is guaranteed that the list represents a number that does not have leading zeros.

 
Follow up: Could you solve it without reversing the input lists?

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
        fn to_stack(mut l: Option<&Box<ListNode>>) -> Vec<i32> {
            let mut st = vec![];
            while let Some(n) = l.as_ref() {
                st.push(n.val);
                l = n.next.as_ref();
            }
            st
        }
        let (mut st1, mut st2) = (to_stack(l1.as_ref()), to_stack(l2.as_ref()));
        let mut next = None;
        let mut c = 0;
        let (mut e1, mut e2) = (false, false);
        loop {
            let x = match st1.pop() {
                Some(v) => v,
                _ => {e1 = true; 0},
            };
            let y = match st2.pop() {
                Some(v) => v,
                _ => {e2 = true; 0},
            };
            if e1 && e2 && c == 0 { break; }
            let s = x + y + c;
            c = s / 10;
            next = Some(Box::new(ListNode{ val: s % 10, next }));

        }
        next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(linked![7,8,0,7], Solution::add_two_numbers(linked![7,2,4,3], linked![5,6,4]));
        assert_eq!(linked![8,0,7], Solution::add_two_numbers(linked![2,4,3], linked![5,6,4]));
        assert_eq!(linked![0], Solution::add_two_numbers(linked![0], linked![0]));
        assert_eq!(linked![1, 0], Solution::add_two_numbers(linked![5], linked![5]));
    }
}
