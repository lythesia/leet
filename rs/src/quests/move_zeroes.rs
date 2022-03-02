/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
Note that you must do this in-place without making a copy of the array.
 
Example 1:
Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]
Example 2:
Input: nums = [0]
Output: [0]
 
Constraints:

	1 <= nums.length <= 104
	-231 <= nums[i] <= 231 - 1

 
Follow up: Could you minimize the total number of operations done?
 */
pub struct Solution {}

// submission codes start here

impl Solution {

    // i to 0, j to none-0, swap until end
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // if let Some(mut i) = nums.iter().position(|&x| x == 0) {
        //     for j in i+1..nums.len() {
        //         if nums[j] != 0 {
        //             nums.swap(i, j);
        //             i += 1;
        //             if i == nums.len() { break; }
        //         }
        //     }
        // }
        let mut zeros = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 { zeros += 1; }
            else {
                if zeros > 0 {
                    nums.swap(i, i - zeros);
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1,3,12,0,0], v);

        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![0], v);
    }
}
