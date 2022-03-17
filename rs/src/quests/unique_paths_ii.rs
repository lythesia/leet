/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
Now consider if some obstacles are added to the grids. How many unique paths would there be?
An obstacle and space is marked as 1 and 0 respectively in the grid.
 
Example 1:
Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
Output: 2
Explanation: There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right

Example 2:
Input: obstacleGrid = [[0,1],[0,0]]
Output: 1

 
Constraints:

	m == obstacleGrid.length
	n == obstacleGrid[i].length
	1 <= m, n <= 100
	obstacleGrid[i][j] is 0 or 1.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i][j] = dp[i - 1][j] if not block + dp[i][j - 1] if not block
    // pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    //     let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    //     let mut dp = vec![vec![0; n]; m];
    //     // start
    //     dp[0][0] = if obstacle_grid[0][0] == 1 {0} else {1};
    //     // 1st row
    //     let stop = obstacle_grid[0].iter().position(|&x| x == 1).unwrap_or(n);
    //     (1..stop).for_each(|i| dp[0][i] = 1);
    //     // 1st col
    //     let stop = (0..m).position(|i| obstacle_grid[i][0] == 1).unwrap_or(m);
    //     (1..stop).for_each(|i| dp[i][0] = 1);

    //     for i in 1..m {
    //         for j in 1..n {
    //             if obstacle_grid[i][j] == 1 {
    //                 dp[i][j] = 0;
    //             } else {
    //                 dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    //             }
    //         }
    //     }
    //     dp[m - 1][n - 1]
    // }

    // or we could reuse input grid
    pub fn unique_paths_with_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 { return 0; }
        let (m, n) = (grid.len(), grid[0].len());
        let block_or = |cell: i32, or_val: i32| {
            if cell == 1 { 0 }
            else { or_val }
        };
        for i in 0..m {
            for j in 0..n {
                grid[i][j] = match (i, j) {
                    (0, 0) => 1,
                    (0, _) => block_or(grid[i][j], grid[i][j - 1]),
                    (_, 0) => block_or(grid[i][j], grid[i - 1][j]),
                    _ => block_or(grid[i][j], grid[i][j - 1] + grid[i - 1][j]),
                }
            }
        }
        grid[m - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::unique_paths_with_obstacles(vec_vec![[0,0,0],[0,1,0],[0,0,0]]));
        assert_eq!(1, Solution::unique_paths_with_obstacles(vec_vec![[0,1],[0,0]]));
    }
}
