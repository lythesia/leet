/**
 * [1020] Number of Enclaves
 *
 * You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.
A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.
Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
 
Example 1:
Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
Output: 3
Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.

Example 2:
Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
Output: 0
Explanation: All 1s are either on the boundary or can reach the boundary.

 
Constraints:

	m == grid.length
	n == grid[i].length
	1 <= m, n <= 500
	grid[i][j] is either 0 or 1.

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = std::collections::VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 { continue; }
                // bfs
                queue.clear();
                let mut edge = false;
                let mut cnt = 0;
                queue.push_back((i, j));
                while let Some((x, y)) = queue.pop_front() {
                    if grid[x][y] == 0 { continue; } // visited
                    grid[x][y] = 0;
                    edge = edge || x == 0 || x == m-1 || y == 0 || y == n-1;
                    cnt += 1;
                    for (dx, dy) in DIR {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == 1 {
                            queue.push_back((nx as usize, ny as usize));
                        }
                    }
                }
                if !edge { ans += cnt; }
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
        assert_eq!(3, Solution::num_enclaves(vec_vec![[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]));
        assert_eq!(0, Solution::num_enclaves(vec_vec![[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]));
    }
}
