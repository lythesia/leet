/**
 * [410] Split Array Largest Sum
 *
 * Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays.
Write an algorithm to minimize the largest sum among these m subarrays.
 
Example 1:
Input: nums = [7,2,5,10,8], m = 2
Output: 18
Explanation:
There are four ways to split nums into two subarrays.
The best way is to split it into [7,2,5] and [10,8],
where the largest sum among the two subarrays is only 18.

Example 2:
Input: nums = [1,2,3,4,5], m = 2
Output: 9

Example 3:
Input: nums = [1,4,4], m = 3
Output: 4

 
Constraints:

	1 <= nums.length <= 1000
	0 <= nums[i] <= 106
	1 <= m <= min(50, nums.length)

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp(i, j): until i-th split into j subarrays
    // dp(i, 1) = prefix_sum[i]
    // dp(i+1, j) = min of max { dp(k, j-1), prefix_sum[i+1] - prefix_sum[k] } for k in [(j-1)-1, i]; a[i+1]和左边(i-k)个一组
    // so:
    // dp(i, j) = min of max pairs
    // dp(i-1, j-1) a[i]
    // dp(i-2, j-1) a[i]+a[i-1]
    // ..
    // dp(j-2, j-1) a[i]+a[i-1]+..+a[j-1]
    // 
    // dp(i-1, j) = min of max pairs
    // dp(i-2, j-1) a[i-1]
    // dp(i-3, j-1) a[i-1]+a[i-2]
    // ..
    // dp(j-2, j-1) a[i-1]+..+a[j-1]
    // ummm, it cannot be reduced ..
    // pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    //     let n = nums.len();
    //     let m = m as usize;
    //     let mut pre = vec![0; n];
    //     let mut dp = vec![vec![i32::MAX; m+1]; n];
    //     for (i, x ) in nums.into_iter().enumerate() {
    //         pre[i] = if i > 0 { pre[i-1] + x } else { x };
    //         dp[i][1] = pre[i]
    //     }
    //     for i in 0..n {
    //         for j in 2..=m {
    //             for k in j-2..i {
    //                 dp[i][j] = dp[i][j].min(
    //                     dp[k][j-1].max(pre[i] - pre[k])
    //                 );
    //             }
    //         }
    //     }
    //     dp[n-1][m]
    // }

    // bin-search like guessing game
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let n = nums.len();
        let m = m as usize;
        let (max, sum) = nums.iter()
            .fold((0, 0), |(m, s), &x| (m.max(x), s + x));
        // if we can split <=m splits each <= s
        let splits_for = |s: i32| {
            if s < max { false }
            else {
                let mut cnt = 1;
                let mut acc = 0;
                for &x in &nums {
                    if acc + x <= s {
                        acc += x;
                    } else {
                        acc = x;
                        cnt += 1;
                        if cnt > m {
                            return false;
                        }
                    }
                }
                true
            }
        };
        let (mut lo, mut hi) = (max, sum);
        while lo < hi {
            let mi = (lo + hi)/2;
            // we can try smaller
            if splits_for(mi) {
                hi = mi;
            // or we should try larger
            } else {
                lo = mi + 1;
            }
        }
        lo
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::split_array(vec![1,1,4], 3));
        assert_eq!(18, Solution::split_array(vec![7,2,5,10,8], 2));
    }
}
