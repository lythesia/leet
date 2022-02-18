/**
 * [2023] Number of Pairs of Strings With Concatenation Equal to Target
 *
 * Given an array of digit strings nums and a digit string target, return the number of pairs of indices (i, j) (where i != j) such that the concatenation of nums[i] + nums[j] equals target.
 
Example 1:
Input: nums = ["777","7","77","77"], target = "7777"
Output: 4
Explanation: Valid pairs are:
- (0, 1): "777" + "7"
- (1, 0): "7" + "777"
- (2, 3): "77" + "77"
- (3, 2): "77" + "77"

Example 2:
Input: nums = ["123","4","12","34"], target = "1234"
Output: 2
Explanation: Valid pairs are:
- (0, 1): "123" + "4"
- (2, 3): "12" + "34"

Example 3:
Input: nums = ["1","1","1"], target = "11"
Output: 6
Explanation: Valid pairs are:
- (0, 1): "1" + "1"
- (1, 0): "1" + "1"
- (0, 2): "1" + "1"
- (2, 0): "1" + "1"
- (1, 2): "1" + "1"
- (2, 1): "1" + "1"

 
Constraints:

	2 <= nums.length <= 100
	1 <= nums[i].length <= 100
	2 <= target.length <= 100
	nums[i] and target consist of digits.
	nums[i] and target do not have leading zeros.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
       let mut ret = 0; 
       let mut h = HashMap::new();

       for s in &nums {
           if let Some(r) = target.strip_prefix(s) {
               if let Some(c) = h.get(r.clone()) {
                   ret += c;
               }
           }
           if let Some(r) = target.strip_suffix(s) {
               if let Some(c) = h.get(r.clone()) {
                   ret += c;
               }
           }

           *h.entry(s.clone()).or_insert(0) += 1;
       }

       ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::num_of_pairs(vec_string!["777","7","77","77"], "7777".to_owned()));
        assert_eq!(2, Solution::num_of_pairs(vec_string!["123","4","12","34"], "1234".to_owned()));
        assert_eq!(6, Solution::num_of_pairs(vec_string!["1","1","1"], "11".to_owned()));
    }
}
