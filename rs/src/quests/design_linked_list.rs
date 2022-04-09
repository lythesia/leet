/**
 * [707] Design Linked List
 *
 * Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
Implement the MyLinkedList class:

	MyLinkedList() Initializes the MyLinkedList object.
	int get(int index) Get the value of the indexth node in the linked list. If the index is invalid, return -1.
	void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
	void addAtTail(int val) Append a node of value val as the last element of the linked list.
	void addAtIndex(int index, int val) Add a node of value val before the indexth node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
	void deleteAtIndex(int index) Delete the indexth node in the linked list, if the index is valid.

 
Example 1:
Input
["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
[[], [1], [3], [1, 2], [1], [1], [1]]
Output
[null, null, null, null, 2, null, 3]
Explanation
MyLinkedList myLinkedList = new MyLinkedList();
myLinkedList.addAtHead(1);
myLinkedList.addAtTail(3);
myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
myLinkedList.get(1);              // return 2
myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
myLinkedList.get(1);              // return 3

 
Constraints:

	0 <= index, val <= 1000
	Please do not use the built-in LinkedList library.
	At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.

 */
pub struct Solution {}

// submission codes start here

use std::{rc::Rc, cell::RefCell};
type Link = Option<Rc<RefCell<ListNode>>>;
struct ListNode {
    val: i32,
    next: Link,
}
impl ListNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            next: None,
        }))
    }
}
struct MyLinkedList {
    head: Link,
    tail: Link,
    len: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        let dummy = ListNode::new(-1);
        Self {
            head: Some(dummy),
            tail: None,
            len: 0,
        }
    }

    fn get_node_at(&self, i: usize) -> Rc<RefCell<ListNode>> {
        let mut curr = self.head.clone();
        for _ in 0..=i {
            curr = curr.unwrap().borrow().next.clone();
        }
        curr.unwrap()
    }
    
    fn get(&self, index: i32) -> i32 {
        let i = index as usize;
        if i >= self.len {
            -1
        } else {
            let node = self.get_node_at(i);
            let ans = node.borrow().val;
            ans
        }
    }
    
    fn add_at_head(&mut self, val: i32) {
        let node = ListNode::new(val);
        if let Some(head) = &self.head {
            {
                let mut head_mut = head.borrow_mut();
                let next = head_mut.next.take();
                node.borrow_mut().next = next;
                head_mut.next = Some(node.clone());
            }
            if self.tail.is_none() {
                self.tail = Some(node);
            }
        }
        self.len += 1;
    }
    
    fn add_at_tail(&mut self, val: i32) {
        if let Some(tail) = self.tail.take() {
            let node = ListNode::new(val);
            tail.borrow_mut().next = Some(node.clone());
            self.tail = Some(node);
            self.len += 1;
        } else {
            self.add_at_head(val);
        }
    }
    
    fn add_at_index(&mut self, index: i32, val: i32) {
        let index = index as usize;
        if index > self.len { return; }
        else if index == self.len {
            self.add_at_tail(val);
        } else if index == 0 {
            self.add_at_head(val);
        } else {
            let inserter = self.get_node_at(index - 1);
            let node = ListNode::new(val);
            let next = inserter.borrow_mut().next.take();
            node.borrow_mut().next = next;
            inserter.borrow_mut().next = Some(node);
            self.len += 1;
        }
    }
    
    fn delete_at_index(&mut self, index: i32) {
        let index = index as usize;
        if index >= self.len { return; }
        let deleter = if index == 0 {
            self.head.clone().unwrap()
        } else {
            self.get_node_at(index - 1)
        };
        let next = deleter.borrow_mut().next.take();
        deleter.borrow_mut().next = next.and_then(|node| node.borrow_mut().next.take());
        self.len -= 1;
        if self.len == 0 {
            self.tail = None;
        } else if self.len == index {
            self.tail = Some(deleter);
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

// submission codes end


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut l = MyLinkedList::new();
        l.add_at_head(7);
        l.add_at_head(2);
        l.add_at_head(1);
        l.add_at_index(3, 0); // 1 2 7 0
        assert_eq!(0, l.get(3));
        l.delete_at_index(2); // 1 2 0
        l.add_at_head(6); // 6 1 2 0
        l.add_at_tail(4); // 6 1 2 0 4
        l.get(4);
        l.add_at_head(4);
        l.add_at_index(5, 0);
        l.add_at_head(6);
    }
}
