/**
 * [994] Rotting Oranges
 *
 * You are given an m x n grid where each cell can have one of three values:

	0 representing an empty cell,
	1 representing a fresh orange, or
	2 representing a rotten orange.

Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
 
Example 1:
Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4

Example 2:
Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
Output: -1
Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.

Example 3:
Input: grid = [[0,2]]
Output: 0
Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.

 
Constraints:

	m == grid.length
	n == grid[i].length
	1 <= m, n <= 10
	grid[i][j] is 0, 1, or 2.

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		use std::collections::VecDeque;
		use std::mem::swap;
		let mut q1 = VecDeque::new();
		let mut q2 = VecDeque::new();
		for i in 0..m {
			for j in 0..n {
				if grid[i][j] == 2 {
					q1.push_back((i, j));
				}
			}
		}
		let mut steps = 0;
		while !q1.is_empty() {
			while let Some((x, y)) = q1.pop_front() {
				for (dx, dy) in DIR {
					let nx = x as i32 + dx;
					let ny = y as i32 + dy;
                	if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == 1 {
						let (nx, ny) = (nx as usize, ny as usize);
						grid[nx][ny] = 2;
                        q2.push_back((nx, ny));
					}
				}
			}
			if !q2.is_empty() {
				steps += 1;
				swap(&mut q1, &mut q2);
			} else {
				break;
			}
		}
		let fresh = (0..m*n).filter(|&k| grid[k/n][k%n] == 1).count();
		if fresh > 0 {
			-1
		} else {
			steps
		}
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert_eq!(4, Solution::oranges_rotting(vec_vec![[2,1,1],[1,1,0],[0,1,1]]));
		assert_eq!(-1, Solution::oranges_rotting(vec_vec![[2,1,1],[0,1,1],[1,0,1]]));
		assert_eq!(0, Solution::oranges_rotting(vec_vec![[0,2]]));
    }
}
