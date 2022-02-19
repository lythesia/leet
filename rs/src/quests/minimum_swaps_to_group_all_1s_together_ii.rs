/**
 * [2134] Minimum Swaps to Group All 1's Together II
 *
 * A swap is defined as taking two distinct positions in an array and swapping the values in them.
A circular array is defined as an array where we consider the first element and the last element to be adjacent.
Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.
 
Example 1:
Input: nums = [0,1,0,1,1,0,0]
Output: 1
Explanation: Here are a few of the ways to group all the 1's together:
[0,0,1,1,1,0,0] using 1 swap.
[0,1,1,1,0,0,0] using 1 swap.
[1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
There is no way to group all 1's together with 0 swaps.
Thus, the minimum number of swaps required is 1.

Example 2:
Input: nums = [0,1,1,1,0,0,1,1,0]
Output: 2
Explanation: Here are a few of the ways to group all the 1's together:
[1,1,1,0,0,0,0,1,1] using 2 swaps (using the circular property of the array).
[1,1,1,1,1,0,0,0,0] using 2 swaps.
There is no way to group all 1's together with 0 or 1 swaps.
Thus, the minimum number of swaps required is 2.

Example 3:
Input: nums = [1,1,0,0,1]
Output: 0
Explanation: All the 1's are already grouped together due to the circular property of the array.
Thus, the minimum number of swaps required is 0.

 
Constraints:

	1 <= nums.length <= 105
	nums[i] is either 0 or 1.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        /*
        let n = nums.len();
        let win: i32 = nums.iter().sum();
        let mut sum: i32 = nums.iter().take(win as usize).sum();
        let mut ret = win - sum;
        let (mut l, mut mmin) = (0usize, win - sum);

        // first window (0, win - 1), sum = sum
        // check every subarray of size = win
        for r in win as usize..(2*n) {
            let rr = r % n; // next r
            let ll = l % n; // last l

            mmin -= (nums[rr] - nums[ll]);
            ret = std::cmp::min(ret, mmin);
            l += 1;
            if l >= n { break; }
        }

        ret
        */
        let ones: i32 = nums.iter().sum();
        let n = nums.len();
        let (mut mmax, mut ones_in_win) = (0, 0);
        for i in 0..2*n {
            // window increase
            if nums[i % n] > 0 { mmax += 1; }
            // window.len > ones, shrink
            if i >= (ones as usize) && nums[(i - (ones as usize)) % n] > 0 { mmax -= 1; }
            ones_in_win = std::cmp::max(ones_in_win, mmax);
        }
        ones - ones_in_win
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::min_swaps(vec![0,1,0,1,1,0,0]));
        assert_eq!(2, Solution::min_swaps(vec![0,1,1,1,0,0,1,1,0]));
        assert_eq!(0, Solution::min_swaps(vec![1,1,0,0,1]));
    }
}
