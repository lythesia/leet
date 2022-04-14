/**
 * [380] Insert Delete GetRandom O(1)
 *
 * Implement the RandomizedSet class:

    RandomizedSet() Initializes the RandomizedSet object.
    bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
    bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
    int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.

You must implement the functions of the class such that each function works in average O(1) time complexity.

Example 1:
Input
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
Output
[null, true, false, true, 2, true, false, 2]
Explanation
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
randomizedSet.insert(2); // 2 was already in the set, so return false.
randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.


Constraints:

    -231 <= val <= 231 - 1
    At most 2 * 105 calls will be made to insert, remove, and getRandom.
    There will be at least one element in the data structure when getRandom is called.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
use rand::Rng;
struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.vec.len());
            self.vec.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.map.remove(&val) {
            // to avoid shift all numbers from index i in vec,
            // we swap vec's last and i and update map
            let last = self.vec.len() - 1;
            if last != i {
                self.vec.swap(i, last);
                if let Some(j) = self.map.get_mut(&self.vec[i]) {
                    *j = i;
                }
            }
            self.vec.pop();
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let ri = rng.gen_range(0..self.vec.len());
        self.vec[ri]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
