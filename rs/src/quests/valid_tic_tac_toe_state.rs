/**
 * [794] Valid Tic-Tac-Toe State
 *
 * Given a Tic-Tac-Toe board as a string array board, return true if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.
The board is a 3 x 3 array that consists of characters ' ', 'X', and 'O'. The ' ' character represents an empty square.
Here are the rules of Tic-Tac-Toe:

	Players take turns placing characters into empty squares ' '.
	The first player always places 'X' characters, while the second player always places 'O' characters.
	'X' and 'O' characters are always placed into empty squares, never filled ones.
	The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
	The game also ends if all squares are non-empty.
	No more moves can be played if the game is over.

 
Example 1:
Input: board = ["O  ","   ","   "]
Output: false
Explanation: The first player always plays "X".

Example 2:
Input: board = ["XOX"," X ","   "]
Output: false
Explanation: Players take turns making moves.

Example 3:
Input: board = ["XOX","O O","XOX"]
Output: true

 
Constraints:

	board.length == 3
	board[i].length == 3
	board[i][j] is either 'X', 'O', or ' '.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
		let mut b = [[' '; 3]; 3];
		let (mut vx, mut vo) = (0, 0);
		for (i, row) in board.into_iter().enumerate() {
			for (j, ch) in row.chars().enumerate() {
				match ch {
					'X' => vx += 1,
					'O' => vo += 1,
					_ => {},
				}
				b[i][j] = ch;
			}
		}

		let is_win = |ch: char| {
			// row
			(0..3).any(|i| (0..3).all(|j| b[i][j] == ch)) ||
			// col
			(0..3).any(|j| (0..3).all(|i| b[i][j] == ch)) ||
			// diag
			b[1][1] == ch && (
				(b[0][0] == ch && b[2][2] == ch) ||
				(b[0][2] == ch && b[2][0] == ch)
			)
		};
		if vx >= vo && vx - vo <= 1 {
			if vx >= 3 && is_win('X') { // x win
				vo == vx - 1 && !is_win('O')
			} else if vx >= 3 { // x no win
				vx == vo && is_win('O') || !is_win('O')
			} else {
				true
			}
		} else { false }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		// assert!(!Solution::valid_tic_tac_toe(vec_string!["O  ","   ","   "]));
		// assert!(!Solution::valid_tic_tac_toe(vec_string!["XOX"," X ","   "]));
		// assert!(Solution::valid_tic_tac_toe(vec_string!["XOX","O O","XOX"]));
		assert!(Solution::valid_tic_tac_toe(vec_string!["XOX","O X","X O"]));
    }
}
