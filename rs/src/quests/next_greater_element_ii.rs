/**
 * [503] Next Greater Element II
 *
 * Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.
The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.
 
Example 1:
Input: nums = [1,2,1]
Output: [2,-1,2]
Explanation: The first 1's next greater number is 2; 
The number 2 can't find next greater number. 
The second 1's next greater number needs to search circularly, which is also 2.

Example 2:
Input: nums = [1,2,3,4,3]
Output: [2,3,4,-1,4]

 
Constraints:

	1 <= nums.length <= 104
	-109 <= nums[i] <= 109

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut mono = vec![];
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut cnt = 0;
        let mut i = 0;
        for k in 0..2*n {
            let i = k%n;
            while let Some(j) = mono.pop() {
                if nums[j] >= nums[i] {
                    mono.push(j); // restore
                    break;
                } else {
                    if ans[j] == -1 {
                        ans[j] = nums[i];
                    }
                }
            }
            mono.push(i);
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![2,-1,2], Solution::next_greater_elements(vec![1,2,1]));
        assert_eq!(vec![2,3,4,-1,4], Solution::next_greater_elements(vec![1,2,3,4,3]));
        assert_eq!(vec![-1,-1,-1], Solution::next_greater_elements(vec![1,1,1]));
    }
}
