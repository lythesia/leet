/**
 * [1306] Jump Game III
 *
 * Given an array of non-negative integers arr, you are initially positioned at start index of the array. When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach to any index with value 0.
Notice that you can not jump outside of the array at any time.
 
Example 1:
Input: arr = [4,2,3,0,3,1,2], start = 5
Output: true
Explanation: 
All possible ways to reach at index 3 with value 0 are: 
index 5 -> index 4 -> index 1 -> index 3 
index 5 -> index 6 -> index 4 -> index 1 -> index 3 

Example 2:
Input: arr = [4,2,3,0,3,1,2], start = 0
Output: true 
Explanation: 
One possible way to reach at index 3 with value 0 is: 
index 0 -> index 4 -> index 1 -> index 3

Example 3:
Input: arr = [3,0,2,1,2], start = 2
Output: false
Explanation: There is no way to reach at index 1 with value 0.

 
Constraints:

	1 <= arr.length <= 5 * 104
	0 <= arr[i] < arr.length
	0 <= start < arr.length

 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let s = start as usize;
        let mut q = VecDeque::new();
        let mut vis = vec![false; arr.len()];
        vis[s] = true;
        q.push_back(s);
        while let Some(i) = q.pop_front() {
            if arr[i] == 0 {
                return true;
            }
            let d = arr[i] as usize;
            if i >= d && !vis[i - d] {
                vis[i - d] = true;
                q.push_back(i - d);
            }
            if i + d < arr.len() && !vis[i + d] {
                vis[i + d] = true;
                q.push_back(i + d);
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
