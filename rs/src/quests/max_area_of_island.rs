/**
 * [695] Max Area of Island
 *
 * You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.
The area of an island is the number of cells with a value 1 in the island.
Return the maximum area of an island in grid. If there is no island, return 0.
 
Example 1:
Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
Output: 6
Explanation: The answer is not 11, because the island must be connected 4-directionally.

Example 2:
Input: grid = [[0,0,0,0,0,0,0,0]]
Output: 0

 
Constraints:

	m == grid.length
	n == grid[i].length
	1 <= m, n <= 50
	grid[i][j] is either 0 or 1.

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32,i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = std::collections::VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 { continue; }
                // bfs
                queue.clear();
                let mut area = 0;
                queue.push_back((i, j));
                while let Some((x, y)) = queue.pop_front() {
                    if grid[x][y] == 0 { continue; }
                    grid[x][y] = 0;
                    area += 1;
                    for (dx, dy) in DIR {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] != 0 {
                            queue.push_back((nx as usize, ny as usize));
                        }
                    }
                }
                ans = std::cmp::max(ans, area);
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
        // assert_eq!(6, Solution::max_area_of_island(vec_vec![
        //     [0,0,1,0,0,0,0,1,0,0,0,0,0],
        //     [0,0,0,0,0,0,0,1,1,1,0,0,0],
        //     [0,1,1,0,1,0,0,0,0,0,0,0,0],
        //     [0,1,0,0,1,1,0,0,1,0,1,0,0],
        //     [0,1,0,0,1,1,0,0,1,1,1,0,0],
        //     [0,0,0,0,0,0,0,0,0,0,1,0,0],
        //     [0,0,0,0,0,0,0,1,1,1,0,0,0],
        //     [0,0,0,0,0,0,0,1,1,0,0,0,0]
        // ]));
        assert_eq!(4, Solution::max_area_of_island(vec_vec![[1,1,0,0,0],[1,1,0,0,0],[0,0,0,1,1],[0,0,0,1,1]]));
    }
}
