/**
 * [542] 01 Matrix
 *
 * Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
The distance between two adjacent cells is 1.
 
Example 1:
Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
Output: [[0,0,0],[0,1,0],[0,0,0]]

Example 2:
Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
Output: [[0,0,0],[0,1,0],[1,2,1]]

 
Constraints:

	m == mat.length
	n == mat[i].length
	1 <= m, n <= 104
	1 <= m * n <= 104
	mat[i][j] is either 0 or 1.
	There is at least one 0 in mat.

 */
pub struct Solution {}

// submission codes start here

use std::cmp::min;
use std::collections::VecDeque;
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut queue = VecDeque::new();
        let mut ans = mat.clone();
        for k in 0..m*n {
            let (i, j) = (k/n, k%n);
            if mat[i][j] == 0 {
                queue.push_back((i, j));
                ans[i][j] = 0;
            } else {
                ans[i][j] = 10010;
            }
        }
        
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >=0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (i, j) = (nx as usize, ny as usize);
                    if mat[i][j] != 0 {
                        mat[i][j] = 0;
                        queue.push_back((i, j));
                        ans[i][j] = min(ans[i][j], ans[x][y] + 1);
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
        // assert_eq!(vec_vec![[0,0,0],[0,1,0],[0,0,0]], Solution::update_matrix(vec_vec![[0,0,0],[0,1,0],[0,0,0]]));
        assert_eq!(vec_vec![[0,0,0],[0,1,0],[1,2,1]],
            Solution::update_matrix(vec_vec![[0,0,0],[0,1,0],[1,1,1]]));
    }
}
