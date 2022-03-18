/**
 * [64] Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.
Note: You can only move either down or right at any point in time.
 
Example 1:
Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
Output: 7
Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.

Example 2:
Input: grid = [[1,2,3],[4,5,6]]
Output: 12

 
Constraints:

	m == grid.length
	n == grid[i].length
	1 <= m, n <= 200
	0 <= grid[i][j] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i][j] = min{ dp[i-1][j], dp[i][j-1]} + a[i][j]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                dp[i][j] = match (i, j) {
                    (0, 0) => 0,
                    (0, _) => dp[0][j - 1],
                    (_, 0) => dp[i - 1][0],
                    _ => std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                } + grid[i][j];
            }
        }
        dp[m - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, Solution::min_path_sum(vec_vec![[1,3,1],[1,5,1],[4,2,1]]));
        assert_eq!(12, Solution::min_path_sum(vec_vec![[1,2,3],[4,5,6]]));
    }
}
