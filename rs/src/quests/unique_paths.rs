/**
 * [62] Unique Paths
 *
 * There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.
The test cases are generated so that the answer will be less than or equal to 2 * 109.
 
Example 1:
Input: m = 3, n = 7
Output: 28

Example 2:
Input: m = 3, n = 2
Output: 3
Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Down -> Down
2. Down -> Down -> Right
3. Down -> Right -> Down

 
Constraints:

	1 <= m, n <= 100

 */
pub struct Solution {}

// submission codes start here

fn ncr(n: i32, r: i32) -> i32 {
    if r > n - r {
        ncr(n, n - r)
    } else {
        // n*..*n-r+1 / 1*..*r
        let x: u128 = ((n-r+1) as u128..=(n as u128)).product();
        let y: u128 = (1..=r as u128).product();
        (x/y) as i32
    }
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // this overflows
        // let fac = |n: u128| -> u128 { (1..=n).product() };
        // let comb = |m, n| (fac(n) / fac(m) / fac(n - m)) as i32; // C(m,n)
        ncr(m + n - 2, n - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4496388, Solution::unique_paths(36, 7));
    }
}
