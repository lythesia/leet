/**
 * [130] Surrounded Regions
 *
 * Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.
A region is captured by flipping all 'O's into 'X's in that surrounded region.
 
Example 1:
Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
Explanation: Surrounded regions should not be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.

Example 2:
Input: board = [["X"]]
Output: [["X"]]

 
Constraints:

	m == board.length
	n == board[i].length
	1 <= m, n <= 200
	board[i][j] is 'X' or 'O'.

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut vis = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        let mut enqueue = |i: usize, j: usize| {
            if board[i][j] == 'O' {
                vis[i][j] = true;
                queue.push_back((i, j));
            }
        };
        // col O
        for i in 0..m {
            for j in [0, n-1] {
                enqueue(i, j);
            }
        }
        // row O
        for j in 0..n {
            for i in [0, m-1] {
                enqueue(i, j);
            }
        }
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >=0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if board[nx][ny] == 'O' && !vis[nx][ny] {
                        vis[nx][ny] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' && !vis[i][j] {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
