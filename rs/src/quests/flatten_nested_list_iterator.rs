/**
 * [341] Flatten Nested List Iterator
 *
 * You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.
Implement the NestedIterator class:

	NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
	int next() Returns the next integer in the nested list.
	boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.

Your code will be tested with the following pseudocode:
initialize iterator with nestedList
res = []
while iterator.hasNext()
    append iterator.next() to the end of res
return res

If res matches the expected flattened list, then your code will be judged as correct.
 
Example 1:
Input: nestedList = [[1,1],2,[1,1]]
Output: [1,1,2,1,1]
Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].

Example 2:
Input: nestedList = [1,[4,[6]]]
Output: [1,4,6]
Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].

 
Constraints:

	1 <= nestedList.length <= 500
	The values of the integers in the nested list is in the range [-106, 106].

 */
pub struct Solution {}

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

use std::collections::VecDeque;
struct NestedIterator {
    queue: VecDeque<NestedInteger>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            queue: VecDeque::from(nested_list),
        }
    }

    fn next(&mut self) -> i32 {
        assert!(self.has_next());

        let head = self.queue.pop_front().unwrap();
        if let NestedInteger::Int(val) = head {
            return val;
        } else {
            unreachable!();
        }
    }
    
    // 核心思想就是总得有一个方法要一直取出list头部元素，如果int就找到了，如果list就flat后插入头部继续找，知道找到第一个int为止
    // 这个逻辑也可以放在next里，但是has_next需要对空串做判断，或者增加一个field-> next: Option<i32>
    fn has_next(&mut self) -> bool {
        while !self.queue.is_empty() {
            if let Some(front) = self.queue.pop_front() {
                match front {
                    NestedInteger::List(mut v) => {
                        for vi in v.into_iter().rev() {
                            self.queue.push_front(vi);
                        }
                    },
                    int => {
                        self.queue.push_front(int);
                        return true;
                    },
                }
            }
        }
        false
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use NestedInteger::{Int,List};
        // [[1,1],2,[1,1]]
        let lst = List(vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ]);

        // [1,[4,[6]]]
        // let lst = List(vec![
        //     Int(1),
        //     List(vec![
        //         Int(4),
        //         List(vec![Int(6)]),
        //     ]),
        // ]);

        // [[]]
        // let lst = List(vec![]);
        let mut it = NestedIterator::new(vec![lst]);
        let mut v = vec![];
        while it.has_next() {
            v.push(it.next());
        }
        println!("{:?}", v);
    }
}
