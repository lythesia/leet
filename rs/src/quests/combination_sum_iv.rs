/**
 * [377] Combination Sum IV
 *
 * Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
The test cases are generated so that the answer can fit in a 32-bit integer.
 
Example 1:
Input: nums = [1,2,3], target = 4
Output: 7
Explanation:
The possible combination ways are:
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)
Note that different sequences are counted as different combinations.

Example 2:
Input: nums = [9], target = 3
Output: 0

 
Constraints:

	1 <= nums.length <= 200
	1 <= nums[i] <= 1000
	All the elements of nums are unique.
	1 <= target <= 1000

 
Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dfs(j) = make j from a[]
    // dfs(j) = sum { dfs(t - a[i]) } for each a[i]
    fn dfs(a: &Vec<i32>, t: usize, memo: &mut Vec<i32>) -> i32 {
        if t == 0 { return 1; }
        if memo[t] != -1 { return memo[t]; }
        else {
            let mut ans = 0;
            for &x in a {
                let xi = x as usize;
                if t >= xi {
                    ans += Self::dfs(a, t - xi, memo);
                }
            }
            memo[t] = ans;
            ans
        }
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // let mut memo = vec![-1; target as usize + 1];
        // Self::dfs(&nums, target as usize, &mut memo)

        // dp ver derived from memo-dfs
        let t = target as usize;
        let mut dp = vec![0; t + 1];
        dp[0] = 1;
        for i in 1..=t {
            // dp[i] = sum { dp[t - a[i]] } for each a[i]
            dp[i] = nums.iter().filter(|&&x| x as usize <= i).map(|&x| dp[i - x as usize]).sum();
        }
        dp[t]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, Solution::combination_sum4(vec![1,2,3], 4));
        assert_eq!(0, Solution::combination_sum4(vec![9], 3));
    }
}
