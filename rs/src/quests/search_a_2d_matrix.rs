/**
 * [74] Search a 2D Matrix
 *
 * Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:

	Integers in each row are sorted from left to right.
	The first integer of each row is greater than the last integer of the previous row.

 
Example 1:
Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
Output: true

Example 2:
Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
Output: false

 
Constraints:

	m == matrix.length
	n == matrix[i].length
	1 <= m, n <= 100
	-104 <= matrix[i][j], target <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let heads = (0..m).map(|i| matrix[i][n-1]).collect::<Vec<_>>();
        match heads.binary_search(&target) {
            Ok(_) => true,
            Err(i) => i < m && matrix[i].binary_search(&target).is_ok(),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::search_matrix(vec_vec![[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3));
        assert!(!Solution::search_matrix(vec_vec![[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13));
    }
}
