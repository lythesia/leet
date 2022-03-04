/**
 * [1162] As Far from Land as Possible
 *
 * Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance. If no land or water exists in the grid, return -1.
The distance used in this problem is the Manhattan distance: the distance between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.
 
Example 1:
Input: grid = [[1,0,1],[0,0,0],[1,0,1]]
Output: 2
Explanation: The cell (1, 1) is as far as possible from all the land with distance 2.

Example 2:
Input: grid = [[1,0,0],[0,0,0],[0,0,0]]
Output: 4
Explanation: The cell (2, 2) is as far as possible from all the land with distance 4.

 
Constraints:

	n == grid.length
	n == grid[i].length
	1 <= n <= 100
	grid[i][j] is 0 or 1

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    // extend from all islands, count steps until any two islands contact
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q1 = std::collections::VecDeque::new();
        let mut q2 = std::collections::VecDeque::new();
        let mut steps = 1;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for (dx, dy) in DIR {
                        let nx = i as i32 + dx;
                        let ny = j as i32 + dy;
                        q1.push_back((nx, ny));
                    }
                }
            }
        }
        while !q1.is_empty() {
            steps += 1;
            while let Some((x, y)) = q1.pop_front() {
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && grid[x as usize][y as usize] == 0 {
                    grid[x as usize][y as usize] = steps;
                    for (dx, dy) in DIR {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        q2.push_back((nx, ny));
                    }
                }
            }
            std::mem::swap(&mut q1, &mut q2);
        }
        if steps == 1 { - 1 }
        else { steps - 1 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::max_distance(vec_vec![[1,0,1],[0,0,0],[1,0,1]]));
        assert_eq!(4, Solution::max_distance(vec_vec![[1,0,0],[0,0,0],[0,0,0]]));
    }
}
