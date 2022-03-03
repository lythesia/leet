/**
 * [1905] Count Sub Islands
 *
 * You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.
An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.
Return the number of islands in grid2 that are considered sub-islands.
 
Example 1:
Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
Output: 3
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

Example 2:
Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
Output: 2 
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.

 
Constraints:

	m == grid1.length == grid2.length
	n == grid1[i].length == grid2[i].length
	1 <= m, n <= 500
	grid1[i][j] and grid2[i][j] are either 0 or 1.

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    fn dfs(g: &mut Vec<Vec<i32>>, m: i32, n: i32, x: usize, y: usize, g0: &Vec<Vec<i32>>) -> bool {
        let mut is_sub = g0[x][y] == 1;
        g[x][y] = 0;
        for (dx, dy) in DIR {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < m && ny >=0 && ny < n && g[nx as usize][ny as usize] != 0 {
                is_sub &= Self::dfs(g, m, n, nx as usize, ny as usize, g0);
            }
        }
        is_sub
    }

    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid1.len(), grid1[0].len());
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    if Self::dfs(&mut grid2, m as i32, n as i32, i, j, &grid1) {
                        ans += 1;
                    }
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
        assert_eq!(3, Solution::count_sub_islands(
            vec_vec![[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]],
            vec_vec![[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]));

        assert_eq!(2, Solution::count_sub_islands(
            vec_vec![[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]],
            vec_vec![[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]));
    }
}
