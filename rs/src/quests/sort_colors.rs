/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
You must solve this problem without using the library's sort function.
 
Example 1:
Input: nums = [2,0,2,1,1,0]
Output: [0,0,1,1,2,2]

Example 2:
Input: nums = [2,0,1]
Output: [0,1,2]

 
Constraints:

	n == nums.length
	1 <= n <= 300
	nums[i] is either 0, 1, or 2.

 
Follow up: Could you come up with a one-pass algorithm using only constant extra space?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // let mut cnt = [0; 3];
        // nums.iter().for_each(|&c| cnt[c as usize] += 1 );
        // nums[0..cnt[0]].fill(0);
        // nums[cnt[0]..cnt[0]+cnt[1]].fill(1);
        // nums[cnt[0]+cnt[1]..].fill(2);

        let (mut r, mut b, mut i) = (0, 0, 0);
        let n = nums.len();
        while i < n - b {
            if nums[i] == 0 {
                nums.swap(i, r);
                r += 1;
            } else if nums[i] == 2{
                nums.swap(i, n-1-b);
                b += 1;
                i -= 1;
            }
            i += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut v);
    }
}
