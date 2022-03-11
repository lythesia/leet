/**
 * [232] Implement Queue using Stacks
 *
 * Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
Implement the MyQueue class:

	void push(int x) Pushes element x to the back of the queue.
	int pop() Removes the element from the front of the queue and returns it.
	int peek() Returns the element at the front of the queue.
	boolean empty() Returns true if the queue is empty, false otherwise.

Notes:

	You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
	Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.

 
Example 1:
Input
["MyQueue", "push", "push", "peek", "pop", "empty"]
[[], [1], [2], [], [], []]
Output
[null, null, null, 1, 1, false]
Explanation
MyQueue myQueue = new MyQueue();
myQueue.push(1); // queue is: [1]
myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
myQueue.peek(); // return 1
myQueue.pop(); // return 1, queue is [2]
myQueue.empty(); // return false

 
Constraints:

	1 <= x <= 9
	At most 100 calls will be made to push, pop, peek, and empty.
	All the calls to pop and peek are valid.

 
Follow-up: Can you implement the queue such that each operation is amortized O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.

 */
pub struct Solution {}

// submission codes start here

struct MyQueue {
    enq: Vec<i32>,
    deq: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            enq: vec![],
            deq: vec![],
        }
    }
    
    fn push(&mut self, x: i32) {
        self.enq.push(x)
    }

    fn enq_to_deq(&mut self) {
        while let Some(top) = self.enq.pop() {
            self.deq.push(top)
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.deq.is_empty() {
            self.enq_to_deq();
        }
        self.deq.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.deq.is_empty() {
            self.enq_to_deq();
        }
        *self.deq.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.enq.is_empty() && self.deq.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
