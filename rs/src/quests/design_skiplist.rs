/**
 * [1206] Design Skiplist
 *
 * Design a Skiplist without using any built-in libraries.
A skiplist is a data structure that takes O(log(n)) time to add, erase and search. Comparing with treap and red-black tree which has the same function and performance, the code length of Skiplist can be comparatively short and the idea behind Skiplists is just simple linked lists.
For example, we have a Skiplist containing [30,40,50,60,70,90] and we want to add 80 and 45 into it. The Skiplist works this way:

Artyom Kalinin [CC BY-SA 3.0], via Wikimedia Commons
You can see there are many layers in the Skiplist. Each layer is a sorted linked list. With the help of the top layers, add, erase and search can be faster than O(n). It can be proven that the average time complexity for each operation is O(log(n)) and space complexity is O(n).
See more about Skiplist: https://en.wikipedia.org/wiki/Skip_list
Implement the Skiplist class:

    Skiplist() Initializes the object of the skiplist.
    bool search(int target) Returns true if the integer target exists in the Skiplist or false otherwise.
    void add(int num) Inserts the value num into the SkipList.
    bool erase(int num) Removes the value num from the Skiplist and returns true. If num does not exist in the Skiplist, do nothing and return false. If there exist multiple num values, removing any one of them is fine.

Note that duplicates may exist in the Skiplist, your code needs to handle this situation.

Example 1:
Input
["Skiplist", "add", "add", "add", "search", "add", "search", "erase", "erase", "search"]
[[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]
Output
[null, null, null, null, false, null, true, false, true, false]
Explanation
Skiplist skiplist = new Skiplist();
skiplist.add(1);
skiplist.add(2);
skiplist.add(3);
skiplist.search(0); // return False
skiplist.add(4);
skiplist.search(1); // return True
skiplist.erase(0);  // return False, 0 is not in skiplist.
skiplist.erase(1);  // return True
skiplist.search(1); // return False, 1 has already been erased.

Constraints:

    0 <= num, target <= 2 * 104
    At most 5 * 104 calls will be made to search, add, and erase.

 */
pub struct Solution {}

// submission codes start here

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

type RefNode = Rc<RefCell<Node>>;
type Link = Option<RefNode>;
struct Node {
    val: i32,
    next: Link,
    down: Link,
}
impl Node {
    fn new(val: i32) -> RefNode {
        Rc::new(RefCell::new(Self {
            val,
            next: None,
            down: None,
        }))
    }
}

struct Skiplist {
    head: RefNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            head: Node::new(-1),
        }
    }

    fn find(&self, val: i32) -> Vec<RefNode> {
        let mut next = Some(self.head.clone());
        let mut v = vec![];
        while let Some(node) = next {
            // 1. see next(right) at first
            // 2. if next <= val: keep going right
            // 3. else > val or no next: down one level (and mark this node as to-be-updated)
            match node.borrow()
                .next
                .clone()
                .filter(|n| n.borrow().val <= val) {
                None => {
                    v.push(node.clone());
                    next = node.borrow().down.clone();
                },
                n => next = n,
            }
        }
        v
    }

    fn search(&self, val: i32) -> bool {
        self.find(val)
            .last()
            .filter(|n| n.borrow().val == val)
            .is_some()
    }

    fn add(&mut self, val: i32) {
        let mut left_nodes = self.find(val);
        let mut new_node = Node::new(val);

        let mut level_zero = None;
        let mut rng = rand::thread_rng();
        while level_zero.is_none() || rng.gen() {
            let left_node = match left_nodes.pop() {
                Some(left) => left,
                _ => { // new one level
                    let new_level_start = Node::new(-1);
                    new_level_start.borrow_mut().down = Some(self.head.clone());
                    self.head = new_level_start;
                    self.head.clone()
                },
            };
            let new_node = Node::new(val);
            new_node.borrow_mut().next = left_node.borrow_mut().next.replace(new_node.clone());
            if let Some(node) = level_zero {
                new_node.borrow_mut().down = Some(node);
            }
            level_zero = Some(new_node);
        }
    }

    fn erase(&mut self, val: i32) -> bool {
        let mut ans = false;
        let mut left_nodes = self.find(val - 1); // should find node exact < val, thus we can erase all nodes of val
        for left_node in left_nodes {
            let next = left_node.borrow_mut().next.take();
            left_node.borrow_mut().next = match next {
                Some(ref n) if n.borrow().val == val => {
                    // but here we removed only one
                    ans = true;
                    n.borrow_mut().next.take()
                },
                _ => next,
            }
        }
        ans
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut skiplist = Skiplist::new();
        skiplist.add(1);
        skiplist.add(2);
        skiplist.add(3);
        assert!(!skiplist.search(0)); // return False
        skiplist.add(4);
        assert!(skiplist.search(1)); // return True
        assert!(!skiplist.erase(0));  // return False, 0 is not in skiplist.
        assert!(skiplist.erase(1));  // return True
        assert!(!skiplist.search(1)); // return False, 1 has already been erased.
    }
}
