/**
 * [384] Shuffle an Array
 *
 * Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.
Implement the Solution class:

	Solution(int[] nums) Initializes the object with the integer array nums.
	int[] reset() Resets the array to its original configuration and returns it.
	int[] shuffle() Returns a random shuffling of the array.

 
Example 1:
Input
["Solution", "shuffle", "reset", "shuffle"]
[[[1, 2, 3]], [], [], []]
Output
[null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
Explanation
Solution solution = new Solution([1, 2, 3]);
solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
                       // Any permutation of [1,2,3] must be equally likely to be returned.
                       // Example: return [3, 1, 2]
solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]

 
Constraints:

	1 <= nums.length <= 50
	-106 <= nums[i] <= 106
	All the elements of nums are unique.
	At most 104 calls in total will be made to reset and shuffle.

 */
// submission codes start here

struct Solution {
    vec: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self { vec: nums, }
    }
    
    fn reset(&self) -> Vec<i32> {
        self.vec.clone()
    }
    
    fn shuffle(&self) -> Vec<i32> {
        use rand::Rng;
        let mut ans = self.vec.clone();
        let mut rng = rand::thread_rng();
        let len = self.vec.len();
        for i in (1..len).rev() {
            let j = rng.gen_range(0..i+1); // rand in [0..i]
            ans.swap(i, j);
        }
        ans
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
