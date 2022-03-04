/**
 * [566] Reshape the Matrix
 *
 * In MATLAB, there is a handy function called reshape which can reshape an m x n matrix into a new one with a different size r x c keeping its original data.
You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.
 
Example 1:
Input: mat = [[1,2],[3,4]], r = 1, c = 4
Output: [[1,2,3,4]]

Example 2:
Input: mat = [[1,2],[3,4]], r = 2, c = 4
Output: [[1,2],[3,4]]

 
Constraints:

	m == mat.length
	n == mat[i].length
	1 <= m, n <= 100
	-1000 <= mat[i][j] <= 1000
	1 <= r, c <= 300

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let (r, c) = (r as usize, c as usize);
        if m*n != r*c { return mat; }
        let mut ans = vec![vec![0; c]; r];
        for i in 0..m {
            for j in 0..n {
                let seq = i*n + j;
                ans[seq/c][seq%c] = mat[i][j];
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
