/**
 * [417] Pacific Atlantic Water Flow
 *
 * There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
 
Example 1:
Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

Example 2:
Input: heights = [[2,1],[1,2]]
Output: [[0,0],[0,1],[1,0],[1,1]]

 
Constraints:

	m == heights.length
	n == heights[r].length
	1 <= m, n <= 200
	0 <= heights[r][c] <= 105

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    fn bfs(h: &mut Vec<Vec<i32>>, i: usize, j: usize, vis: &mut [[bool;200];200]) {
        let (m, n) = (h.len(), h[0].len());
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((i, j));
        vis[i][j] = true;
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 &&
                    !vis[nx as usize][ny as usize] && h[nx as usize][ny as usize] >= h[x][y] {
                        vis[nx as usize][ny as usize] = true;
                        queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    pub fn pacific_atlantic(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut reach_pac = [[false; 200]; 200];
        let mut reach_atl = [[false; 200]; 200];

        let (m, n) = (heights.len(), heights[0].len());
        for i in 0..m {
            Self::bfs(&mut heights, i, 0, &mut reach_pac); // pacific coast col
            Self::bfs(&mut heights, i, n-1, &mut reach_atl); // atlantic coast col
        }
        for j in 0..n {
            Self::bfs(&mut heights, 0, j, &mut reach_pac); // pacific coast row
            Self::bfs(&mut heights, m-1, j, &mut reach_atl); // atlantic coast row
        }
        for i in 0..m {
            for j in 0..n {
                if reach_pac[i][j] && reach_atl[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
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
        // assert_eq!(vec_vec![[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]],
        // Solution::pacific_atlantic(vec_vec![[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]));

        // assert_eq!(vec_vec![[0,0],[0,1],[1,0],[1,1]],
        // Solution::pacific_atlantic(vec_vec![[2,1],[1,2]]));
    }
}
