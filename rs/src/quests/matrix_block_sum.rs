/**
 * [1314] Matrix Block Sum
 *
 * Given a m x n matrix mat and an integer k, return a matrix answer where each answer[i][j] is the sum of all elements mat[r][c] for:

	i - k <= r <= i + k,
	j - k <= c <= j + k, and
	(r, c) is a valid position in the matrix.

 
Example 1:
Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
Output: [[12,21,16],[27,45,33],[24,39,28]]

Example 2:
Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
Output: [[45,45,45],[45,45,45],[45,45,45]]

 
Constraints:

	m == mat.length
	n == mat[i].length
	1 <= m, n, k <= 100
	1 <= mat[i][j] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
		let (m, n) = (mat.len(), mat[0].len());
		let mut prefix = vec![vec![0; n]; m];
		let mut ans = prefix.clone();
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
		let area_to = |(i, j): (i32, i32)| {
			if i < 0 || j < 0 { 0 }
			else {
				prefix[i.clamp(0, m as i32 - 1) as usize][j.clamp(0, n as i32 - 1) as usize]
			}
		};
		let block_sum = |i: i32, j: i32, k: i32| {
			let tot = ((i + k), (j + k));
			let x = ((i + k), (j - k - 1));
			let y = ((i - k - 1), (j + k));
			let z = ((i - k - 1), (j - k - 1));
			area_to(tot) - area_to(x) - area_to(y) + area_to(z)
		};
		for i in 0..m {
			for j in 0..n {
				ans[i][j] = block_sum(i as i32, j as i32, k);
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
		assert_eq!(vec_vec![[12,21,16],[27,45,33],[24,39,28]], Solution::matrix_block_sum(vec_vec![[1,2,3],[4,5,6],[7,8,9]], 1));
		assert_eq!(vec_vec![[45,45,45],[45,45,45],[45,45,45]], Solution::matrix_block_sum(vec_vec![[1,2,3],[4,5,6],[7,8,9]], 2));
    }
}
