/**
 * [48] Rotate Image
 *
 * You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
 
Example 1:
Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [[7,4,1],[8,5,2],[9,6,3]]

Example 2:
Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

 
Constraints:

	n == matrix.length == matrix[i].length
	1 <= n <= 20
	-1000 <= matrix[i][j] <= 1000

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // transpose(flip by diag) + flip by verticle
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        matrix.reverse();
        for i in 0..n {
            for j in i+1..n {
                matrix[i][j] ^= matrix[j][i];
                matrix[j][i] = matrix[i][j] ^ matrix[j][i];
                matrix[i][j] ^= matrix[j][i];
            }
        }
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
