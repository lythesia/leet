/**
 * [79] Word Search
 *
 * Given an m x n grid of characters board and a string word, return true if word exists in the grid.
The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
 
Example 1:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true

Example 2:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true

Example 3:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false

 
Constraints:

	m == board.length
	n = board[i].length
	1 <= m, n <= 6
	1 <= word.length <= 15
	board and word consists of only lowercase and uppercase English letters.

 
Follow up: Could you use search pruning to make your solution faster with a larger board?

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32,i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
fn dfs(s: &[u8], x: usize, y: usize, m: usize, n: usize, board: &mut Vec<Vec<char>>) -> bool {
    if s.is_empty() { return true; }
    let ch = s[0] as char;

    for (dx, dy) in DIR {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
            let nx = nx as usize;
            let ny = ny as usize;
            let next = &mut board[nx][ny];
            if *next == ch {
                let old = *next;
                board[nx][ny] = '*';
                if dfs(&s[1..], nx, ny, m, n, board) { return true; }
                board[nx][ny] = old;
            }
        }
    }
    false
}
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.as_bytes();
        let s = word[0] as char;
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            for j in 0..n {
                if s == board[i][j] {
                    let old = board[i][j];
                    board[i][j] = '*';
                    if dfs(&word[1..], i, j, m, n, &mut board) {
                        return true;
                    }
                    board[i][j] = old;
                }
            }
        }
        false
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
