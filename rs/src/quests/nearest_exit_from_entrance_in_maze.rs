/**
 * [1926] Nearest Exit from Entrance in Maze
 *
 * You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.
In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance does not count as an exit.
Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.
 
Example 1:
Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
Output: 1
Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
Initially, you are at the entrance cell [1,2].
- You can reach [1,0] by moving 2 steps left.
- You can reach [0,2] by moving 1 step up.
It is impossible to reach [2,3] from the entrance.
Thus, the nearest exit is [0,2], which is 1 step away.

Example 2:
Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
Output: 2
Explanation: There is 1 exit in this maze at [1,2].
[1,0] does not count as an exit since it is the entrance cell.
Initially, you are at the entrance cell [1,0].
- You can reach [1,2] by moving 2 steps right.
Thus, the nearest exit is [1,2], which is 2 steps away.

Example 3:
Input: maze = [[".","+"]], entrance = [0,0]
Output: -1
Explanation: There are no exits in this maze.

 
Constraints:

	maze.length == m
	maze[i].length == n
	1 <= m, n <= 100
	maze[i][j] is either '.' or '+'.
	entrance.length == 2
	0 <= entrancerow < m
	0 <= entrancecol < n
	entrance will always be an empty cell.

 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
		let (m, n) = (maze.len(), maze[0].len());
		let (sx, sy) = (entrance[0] as usize, entrance[1] as usize);
		let mut queue = VecDeque::new();
		let mut step = vec![vec![0; n]; m];
		maze[sx][sy] = '*';
		queue.push_back((sx, sy));
		while let Some((x, y)) = queue.pop_front() {
			for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (i, j) = (nx as usize, ny as usize);
					// not visit
					if maze[i][j] == '.' {
						maze[i][j] = '*';
						step[i][j] = step[x][y] + 1;
						queue.push_back((i, j));
					}
				} else if x != sx || y != sy {
					// out of border
					return step[x][y];
				}
			}
		}
		-1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert_eq!(1, Solution::nearest_exit(vec_vec![['+','+','.','+'],['.','.','.','+'],['+','+','+','.']], vec![1,2]));
		assert_eq!(2, Solution::nearest_exit(vec_vec![['+','+','+'],['.','.','.'],['+','+','+']], vec![1,0]));
		assert_eq!(-1, Solution::nearest_exit(vec_vec![['.','+']], vec![0, 0]));
    }
}
