/**
 * [155] Min Stack
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
Implement the MinStack class:

	MinStack() initializes the stack object.
	void push(int val) pushes the element val onto the stack.
	void pop() removes the element on the top of the stack.
	int top() gets the top element of the stack.
	int getMin() retrieves the minimum element in the stack.

 
Example 1:
Input
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]
Output
[null,null,null,null,-3,null,0,-2]
Explanation
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2

 
Constraints:

	-231 <= val <= 231 - 1
	Methods pop, top and getMin operations will always be called on non-empty stacks.
	At most 3 * 104 calls will be made to push, pop, top, and getMin.

 */
pub struct Solution {}

// submission codes start here

struct MinStack {
    stack: Vec<i32>,
    min_idx: Vec<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self { stack: vec![], min_idx: vec![], }
    }
    
    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push(val);
            self.min_idx.push(0);
        } else {
            let min = self.stack[*self.min_idx.last().unwrap()];
            self.stack.push(val);
            if val < min {
                self.min_idx.push(self.stack.len() - 1);
            }
        }
    }
    
    fn pop(&mut self) {
        let back = self.stack.pop().unwrap();
        if *self.min_idx.last().unwrap() == self.stack.len() {
            self.min_idx.pop();
        }
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.stack[*self.min_idx.last().unwrap()]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
