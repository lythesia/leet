/**
 * [304] Range Sum Query 2D - Immutable
 *
 * Given a 2D matrix matrix, handle multiple queries of the following type:

	Calculate the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).

Implement the NumMatrix class:

	NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
	int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).

 
Example 1:
Input
["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
[[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
Output
[null, 8, 11, 12]
Explanation
NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)

 
Constraints:

	m == matrix.length
	n == matrix[i].length
	1 <= m, n <= 200
	-105 <= matrix[i][j] <= 105
	0 <= row1 <= row2 < m
	0 <= col1 <= col2 < n
	At most 104 calls will be made to sumRegion.

 */
pub struct Solution {}

// submission codes start here

struct NumMatrix {
	prefix: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(mat: Vec<Vec<i32>>) -> Self {
		let (m, n) = (mat.len(), mat[0].len());
		let mut prefix = vec![vec![0; n]; m];
		prefix[0][0] = mat[0][0];
		for i in 0..m {
			let mut row_prefix = vec![0; n];
			row_prefix[0] = mat[i][0];
			if i > 0 {
				prefix[i][0] = prefix[i - 1][0] + mat[i][0];
			}
			for j in 1..n {
				row_prefix[j] = row_prefix[j - 1] + mat[i][j];
				if i == 0 {
					prefix[0][j] = row_prefix[j];
				} else {
					prefix[i][j] = prefix[i-1][j] + row_prefix[j];
				}
			}
		}
		Self { prefix }
    }

	fn area_to(&self, (i, j): (i32, i32)) -> i32 {
		let (m, n) = (self.prefix.len(), self.prefix[0].len());
		if i < 0 || j < 0 { 0 }
		else {
			self.prefix[i.clamp(0, m as i32 - 1) as usize][j.clamp(0, n as i32 - 1) as usize]
		}
	}
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
		let tot = (row2, col2);
		let x = (row2, (col1 - 1));
		let y = ((row1 - 1), col2);
		let z = ((row1 - 1), (col1 - 1));
		self.area_to(tot) - self.area_to(x) - self.area_to(y) + self.area_to(z)
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
