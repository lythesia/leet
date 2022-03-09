/**
 * [934] Shortest Bridge
 *
 * You are given an n x n binary matrix grid where 1 represents land and 0 represents water.
An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.
You may change 0's to 1's to connect the two islands to form one island.
Return the smallest number of 0's you must flip to connect the two islands.
 
Example 1:
Input: grid = [[0,1],[1,0]]
Output: 1

Example 2:
Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
Output: 2

Example 3:
Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
Output: 1

 
Constraints:

	n == grid.length == grid[i].length
	2 <= n <= 100
	grid[i][j] is either 0 or 1.
	There are exactly two islands in grid.

 */
pub struct Solution {}

// submission codes start here


use std::collections::BinaryHeap;
use std::cmp::Reverse;
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![false; n]; n];
        let mut queue = BinaryHeap::new();
        let mut start = (0, 0, 0);
        for k in 0..n*n {
            let (i, j) = (k/n, k%n);
            if grid[i][j] != 0 {
                start = (grid[i][j], i, j);
                vis[i][j] = true;
                break;
            }
        }
        let mut ans = 0;
        queue.push(Reverse(start));
        while let Some(Reverse((d, x,y))) = queue.pop() {
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                    let (i, j) = (nx as usize, ny as usize);
                    if grid[i][j] == 0 {
                        grid[i][j] = d + 1;
                        ans = std::cmp::max(ans, grid[i][j]);
                    } else if grid[i][j] == 1 && !vis[i][j] && grid[x][y] > 1 {
                        return grid[x][y] - 1;
                    }
                    if !vis[i][j] {
                        vis[i][j] = true;
                        let next= (grid[i][j], i, j);
                        queue.push(Reverse(next));
                    }
                }
            }
        }
        ans - 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::shortest_bridge(vec_vec![[0,1],[1,0]]));
        assert_eq!(2, Solution::shortest_bridge(vec_vec![[0,1,0],[0,0,0],[0,0,1]]));
        assert_eq!(1, Solution::shortest_bridge(vec_vec![[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]));
        assert_eq!(2, Solution::shortest_bridge(vec_vec![[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,1,0],[0,0,1,1,0,0,1,1],[0,0,0,1,1,0,0,1],[0,0,1,1,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0]]))
    }
}
