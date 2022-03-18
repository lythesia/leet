/**
 * [221] Maximal Square
 *
 * Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 
Example 1:
Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
Output: 4

Example 2:
Input: matrix = [["0","1"],["1","0"]]
Output: 1

Example 3:
Input: matrix = [["0"]]
Output: 0

 
Constraints:

	m == matrix.length
	n == matrix[i].length
	1 <= m, n <= 300
	matrix[i][j] is '0' or '1'.

 */
pub struct Solution {}

// submission codes start here

use std::cmp::{min, max};
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                match (i, j) {
                    (0, _) | (_, 0) => if matrix[i][j] == '1' {
                        dp[i][j] = 1;
                    },
                    _ => if matrix[i][j] == '1' {
                        dp[i][j] = 1 + min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1]));
                    },
                };
                ans = max(ans, dp[i][j]);
            }
        }
        ans*ans
    }
    // pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    //     let (m, n) = (matrix.len(), matrix[0].len());
    //     let mut height = vec![0; n];
    //     let (mut left, mut right) = (height.clone(), height.clone());
    //     let mut ans = 0;
    //     for i in 0..m {
    //         // calc(inc) height
    //         for j in 0..n {
    //             height[j] = if matrix[i][j] == '1' { height[j] + 1 } else { 0 };
    //         }
    //         // re-calc left
    //         left[0] = 0;
    //         for j in 1..n {
    //             left[j] = j;
    //             while left[j] >= 1 && height[j] <= height[left[j] - 1] {
    //                 left[j] = left[left[j] - 1];
    //             }
    //         }
    //         // re-calc right
    //         right[n - 1] = n - 1;
    //         for j in (0..n-1).rev() {
    //             right[j] = j;
    //             while right[j] < n - 1 && height[j] <= height[right[j] + 1] {
    //                 right[j] = right[right[j] + 1];
    //             }
    //         }
    //         ans = max(ans,
    //             (0..n).map(|j| min(height[j], right[j] - left[j] + 1).pow(2)).max().unwrap() as i32);
    //     }
    //     ans
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
