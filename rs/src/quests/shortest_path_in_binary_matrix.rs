/**
 * [1091] Shortest Path in Binary Matrix
 *
 * Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:

	All the visited cells of the path are 0.
	All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).

The length of a clear path is the number of visited cells of this path.
 
Example 1:
Input: grid = [[0,1],[1,0]]
Output: 2

Example 2:
Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
Output: 4

Example 3:
Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
Output: -1

 
Constraints:

	n == grid.length
	n == grid[i].length
	1 <= n <= 100
	grid[i][j] is 0 or 1

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 8] = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)];
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = std::collections::VecDeque::new();
        if grid[0][0] != 0 || grid[m-1][n-1] != 0 { return -1; }
        grid[0][0] = 1;
        queue.push_back((0, 0));
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >=0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 0 { // can visit
                        grid[nx][ny] = grid[x][y] + 1;
                        if nx == m - 1 && ny == n - 1 {
                            return grid[nx][ny];
                        }
                        queue.push_back((nx as usize, ny as usize));
                    }
                }
            }
        }
        if grid[m-1][n-1] > 0 { grid[m-1][n-1] }
        else { -1 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(4, Solution::shortest_path_binary_matrix(vec_vec![[0,0,0],[1,1,0],[1,1,0]]));
        // assert_eq!(-1, Solution::shortest_path_binary_matrix(vec_vec![[1,0,0],[1,1,0],[1,1,0]]));
        assert_eq!(-1, Solution::shortest_path_binary_matrix(vec_vec![[0,0,0],[1,1,0],[1,1,1]]));
    }
}
