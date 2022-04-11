/**
 * [1260] Shift 2D Grid
 *
 * Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
In one shift operation:

	Element at grid[i][j] moves to grid[i][j + 1].
	Element at grid[i][n - 1] moves to grid[i + 1][0].
	Element at grid[m - 1][n - 1] moves to grid[0][0].

Return the 2D grid after applying shift operation k times.
 
Example 1:
Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
Output: [[9,1,2],[3,4,5],[6,7,8]]

Example 2:
Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]

Example 3:
Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
Output: [[1,2,3],[4,5,6],[7,8,9]]

 
Constraints:

	m == grid.length
	n == grid[i].length
	1 <= m <= 50
	1 <= n <= 50
	-1000 <= grid[i][j] <= 1000
	0 <= k <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
		let (m, n, mut k) = (grid.len(), grid[0].len(), k as usize);
		let row_cycles = k/n%m;
		k %= n;
		// shift row down
		let mut ans = grid.split_off(m - row_cycles);
		ans.extend(grid);
		// shift col and deal first
		for _ in 0..k {
			let mut last_col = ans.iter_mut().map(|row| row.pop().unwrap()).collect::<Vec<_>>();
			let tail = last_col.pop().unwrap();
			last_col.insert(0, tail);
			ans.iter_mut().zip(last_col).for_each(|(row, x)| row.insert(0, x));
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
		println!("{:?}", Solution::shift_grid(vec_vec![[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], 4));
		println!("{:?}", Solution::shift_grid(vec_vec![[1,2,3],[4,5,6],[7,8,9]], 9));
    }
}
