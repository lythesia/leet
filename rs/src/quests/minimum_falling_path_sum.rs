/**
 * [931] Minimum Falling Path Sum
 *
 * Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.
A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).
 
Example 1:
Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
Output: 13
Explanation: There are two falling paths with a minimum sum as shown.

Example 2:
Input: matrix = [[-19,57],[-40,-5]]
Output: -59
Explanation: The falling path with a minimum sum is shown.

 
Constraints:

	n == matrix.length == matrix[i].length
	1 <= n <= 100
	-100 <= matrix[i][j] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let n = matrix.len();
        let mut dp = vec![vec![0; n]; 2];
        let mut row = 0;
        dp[row].clone_from(&matrix[0]);
        for i in 1..n {
            let a = &matrix[i];
            for j in 0..n {
                dp[1 - row][j] = if j == 0 {
                    min(dp[row][j], dp[row][j + 1])
                } else if j == n - 1{
                    min(dp[row][j - 1], dp[row][j])
                } else {
                    min(dp[row][j - 1], min(dp[row][j], dp[row][j + 1]))
                } + a[j];
            }
            row = 1 - row;
        }
        *dp[row].iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(13, Solution::min_falling_path_sum(vec_vec![[2,1,3],[6,5,4],[7,8,9]]));
        assert_eq!(-59, Solution::min_falling_path_sum(vec_vec![[-19,57],[-40,-5]]));
        assert_eq!(-36, Solution::min_falling_path_sum(vec_vec![[100,-42,-46,-41],[31,97,10,-10],[-58,-51,82,89],[51,81,69,-51]]));
    }
}
