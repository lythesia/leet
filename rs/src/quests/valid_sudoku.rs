/**
 * [36] Valid Sudoku
 *
 * Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

	Each row must contain the digits 1-9 without repetition.
	Each column must contain the digits 1-9 without repetition.
	Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

Note:

	A Sudoku board (partially filled) could be valid but is not necessarily solvable.
	Only the filled cells need to be validated according to the mentioned rules.

 
Example 1:
Input: board = 
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: true

Example 2:
Input: board = 
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.

 
Constraints:

	board.length == 9
	board[i].length == 9
	board[i][j] is a digit 1-9 or '.'.

 */
pub struct Solution {}

// submission codes start here

const IDX: [(usize, usize); 9] = [
	(0,0), (0,3), (0,6),
    (3,0), (3,3), (3,6),
    (6,0), (6,3), (6,6)
];
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
		let helper = |chk: &mut [i32; 9], ch: char| -> bool {
			if ch == '.' { true }
			else {
				let i = ch as usize - '1' as usize;
				chk[i] += 1;
				chk[i] == 1
			}
		};
		(0..9).all(|i| {
			let mut chk = [0;9];
			let iter_chk = |mut it: Box<dyn Iterator<Item=&char>>, chk: &mut [i32; 9]| {
				it.try_fold({*chk = [0;9]; true}, |_, &ch| if helper(chk, ch) {Ok(true)} else {Err(())})
					.unwrap_or(false)
			};
			// each row
			iter_chk(Box::new((0..9).map(|j| &board[i][j])), &mut chk) &&
			// each col
			iter_chk(Box::new((0..9).map(|j| &board[j][i])), &mut chk) &&
			// each square
			iter_chk(Box::new((0..9).map(|k| &board[IDX[i].0 + k/3][IDX[i].1 + k%3])), &mut chk)
		})
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert!(Solution::is_valid_sudoku(vec_vec![
			['5','3','.','.','7','.','.','.','.']
			,['6','.','.','1','9','5','.','.','.']
			,['.','9','8','.','.','.','.','6','.']
			,['8','.','.','.','6','.','.','.','3']
			,['4','.','.','8','.','3','.','.','1']
			,['7','.','.','.','2','.','.','.','6']
			,['.','6','.','.','.','.','2','8','.']
			,['.','.','.','4','1','9','.','.','5']
			,['.','.','.','.','8','.','.','7','9']
		]));

		assert!(!Solution::is_valid_sudoku(vec_vec![
			['8','3','.','.','7','.','.','.','.']
			,['6','.','.','1','9','5','.','.','.']
			,['.','9','8','.','.','.','.','6','.']
			,['8','.','.','.','6','.','.','.','3']
			,['4','.','.','8','.','3','.','.','1']
			,['7','.','.','.','2','.','.','.','6']
			,['.','6','.','.','.','.','2','8','.']
			,['.','.','.','4','1','9','.','.','5']
			,['.','.','.','.','8','.','.','7','9']
		]));
    }
}
