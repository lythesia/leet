/**
 * [1588] Sum of All Odd Length Subarrays
 *
 * Given an array of positive integers arr, calculate the sum of all possible odd-length subarrays.
A subarray is a contiguous subsequence of the array.
Return the sum of all odd-length subarrays of arr.
 
Example 1:
Input: arr = [1,4,2,5,3]
Output: 58
Explanation: The odd-length subarrays of arr and their sums are:
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
If we add all these together we get 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
Example 2:
Input: arr = [1,2]
Output: 3
Explanation: There are only 2 subarrays of odd length, [1] and [2]. Their sum is 3.
Example 3:
Input: arr = [10,11,12]
Output: 66

 
Constraints:

	1 <= arr.length <= 100
	1 <= arr[i] <= 1000

 */
pub struct Solution {}

// submission codes start here
use std::cmp::{min, max};
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        // let n = arr.len();
        // let k = (n + 1)/2; // k groups
        // let mut ans = 0;
        // for (i, &x) in arr.iter().enumerate() {
        //     // group len: m in [1,3,..]
        //     for m in (1..=2*k-1).step_by(2) {
        //         // cnt# occurs in each group
        //         let start_lo = max(0, i as i32 - m as i32 + 1);
        //         let start_hi = min(i as i32, n as i32 - m as i32);
        //         ans += x*(start_hi - start_lo + 1);
        //     }
        // }
        // ans

        // nice ver
        // subarray containing a[i], there's
        // i + 1 start indices to the left (include i), and n - i end indices to the right
        // we need to count pairs of (i+1, n-i) to make it odd
        // in total there's (i+1)*(n-i) pairs, but odd is always half or about half, which is
        // ((i+1)*(n-i) + 1)/2, that's the total occurances of a[i]
        let n = arr.len();
        arr.into_iter().enumerate().map(|(i, x)| x*(((i + 1)*(n - i) + 1)/2) as i32).sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(58, Solution::sum_odd_length_subarrays(vec![1,4,2,5,3]));
        assert_eq!(3, Solution::sum_odd_length_subarrays(vec![1,2]));
        assert_eq!(66, Solution::sum_odd_length_subarrays(vec![10,11,12]));
    }
}
