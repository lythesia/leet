/**
 * [120] Triangle
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
 
Example 1:
Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
Output: 11
Explanation: The triangle looks like:
   2
  3 4
 6 5 7
4 1 8 3
The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).

Example 2:
Input: triangle = [[-10]]
Output: -10

 
Constraints:

	1 <= triangle.length <= 200
	triangle[0].length == 1
	triangle[i].length == triangle[i - 1].length + 1
	-104 <= triangle[i][j] <= 104

 
Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let n = triangle.len();
        let mut dp = vec![vec![0; n]; 2];
        let mut row = 0;
        dp[row][0] = triangle[0][0];
        for i in 1..n {
            for j in (0..=i).rev() {
                dp[1-row][j] = if j == i {
                    dp[row][j - 1]
                } else if j == 0 {
                    dp[row][j]
                } else {
                    min(dp[row][j - 1], dp[row][j])
                } + triangle[i][j];
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
        assert_eq!(11, Solution::minimum_total(vec_vec![[2],[3,4],[6,5,7],[4,1,8,3]]));
        assert_eq!(-10, Solution::minimum_total(vec_vec![[-10]]));
    }
}
