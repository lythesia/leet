/**
 * [54] Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 
Example 1:
Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [1,2,3,6,9,8,7,4,5]

Example 2:
Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
Output: [1,2,3,4,8,12,11,10,9,5,6,7]

 
Constraints:

	m == matrix.length
	n == matrix[i].length
	1 <= m, n <= 10
	-100 <= matrix[i][j] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut tot = m*n - 1;
        let (mut x, mut y) = (0, 0);
        let mut ans = Vec::with_capacity(tot + 1);
        ans.push(matrix[x][y]);
        while tot > 0 {
            while y + 1 < n - x && tot > 0 { ans.push(matrix[x][y+1]); y += 1; tot -= 1; } // go right
            while x + 1 < m - (n - y - 1) && tot > 0 { ans.push(matrix[x+1][y]); x += 1; tot -= 1; } // go down
            while y >= 1 && y - 1 >= (m - x) - 1 && tot > 0 { ans.push(matrix[x][y-1]); y -= 1; tot -= 1; } // go left
            while x - 1 > y && tot > 0 { ans.push(matrix[x-1][y]); x -= 1; tot -= 1; } // go up
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
