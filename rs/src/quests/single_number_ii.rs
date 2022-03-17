/**
 * [137] Single Number II
 *
 * Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
You must implement a solution with a linear runtime complexity and use only constant extra space.
 
Example 1:
Input: nums = [2,2,3,2]
Output: 3
Example 2:
Input: nums = [0,1,0,1,0,1,99]
Output: 99
 
Constraints:

	1 <= nums.length <= 3 * 104
	-231 <= nums[i] <= 231 - 1
	Each element in nums appears exactly three times except for one element which appears once.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // found formula:
    // a ^ b = a&!b | !a&b
    // a ^ b = b ^ a; (a ^ b) ^ c = a ^ (b ^ c)
    // a & (b ^ c) = a&b ^ a&c
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // there's fact: one_time & two_times = 0, they never overlap
        let mut bits_acc_one_time = 0;
        let mut bits_acc_two_times = 0;
        let mut bits_acc_three_times = 0;
        nums.into_iter().for_each(|a| {
            let inc_one_time_to_two_times = a & bits_acc_one_time;
            // (next_one_time, next_two_time, next_three_time) = 
            // (one_time ^ a, two_times | inc, (one_time ^ a) & two_times == a & two_times)
            bits_acc_three_times = bits_acc_two_times & a;
            bits_acc_two_times |= inc_one_time_to_two_times; // or two_times ^= one_time, also ok
            bits_acc_one_time ^= a;
            // reset if reach three times
            bits_acc_two_times &= !bits_acc_three_times;
            bits_acc_one_time &= !bits_acc_three_times;
        });
        bits_acc_one_time
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::single_number(vec![2,2,3,2]));
        assert_eq!(99, Solution::single_number(vec![0,1,0,1,0,1,99]));
    }
}
