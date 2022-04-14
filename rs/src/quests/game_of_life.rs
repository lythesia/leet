/**
 * [289] Game of Life
 *
 * According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

	Any live cell with fewer than two live neighbors dies as if caused by under-population.
	Any live cell with two or three live neighbors lives on to the next generation.
	Any live cell with more than three live neighbors dies, as if by over-population.
	Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.
 
Example 1:
Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]

Example 2:
Input: board = [[1,1],[1,0]]
Output: [[1,1],[1,1]]

 
Constraints:

	m == board.length
	n == board[i].length
	1 <= m, n <= 25
	board[i][j] is 0 or 1.

 
Follow up:

	Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.
	In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32,i32); 8] = [
	(-1,-1),(-1,0),(-1,1),
	(0,-1),(0,1),
	(1,-1),(1,0),(1,1),
];
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
		let (m, n) = (board.len(), board[0].len());
		for i in 0..m {
			for j in 0..n {
				let mut live = 0;
				for (dx, dy) in DIR {
					let nx = i as i32 + dx;
					let ny = j as i32 + dy;
					if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
						let (nx, ny) = (nx as usize, ny as usize);
						if board[nx][ny] & 1 == 1 { live += 1; }
					}
				}
				if board[i][j] & 1 == 1 && (live == 2 || live == 3)
				|| live == 3 {
					board[i][j] ^= 0b10;
				}
			}
		}
		for i in 0..m {
			for j in 0..n {
				board[i][j] = if board[i][j] & 0b10 > 0 {
					1
				} else {
					0
				};
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
		// let mut v = vec_vec![[0,1,0],[0,0,1],[1,1,1],[0,0,0]];
		let mut v = vec_vec![[1,1],[1,0]];
		println!("{:?}", {Solution::game_of_life(&mut v); v});
    }
}
