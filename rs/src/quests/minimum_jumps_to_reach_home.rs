/**
 * [1654] Minimum Jumps to Reach Home
 *
 * A certain bug's home is on the x-axis at position x. Help them get there from position 0.
The bug jumps according to the following rules:

	It can jump exactly a positions forward (to the right).
	It can jump exactly b positions backward (to the left).
	It cannot jump backward twice in a row.
	It cannot jump to any forbidden positions.

The bug may jump forward beyond its home, but it cannot jump to positions numbered with negative integers.
Given an array of integers forbidden, where forbidden[i] means that the bug cannot jump to the position forbidden[i], and integers a, b, and x, return the minimum number of jumps needed for the bug to reach its home. If there is no possible sequence of jumps that lands the bug on position x, return -1.
 
Example 1:
Input: forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9
Output: 3
Explanation: 3 jumps forward (0 -> 3 -> 6 -> 9) will get the bug home.

Example 2:
Input: forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11
Output: -1

Example 3:
Input: forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7
Output: 2
Explanation: One jump forward (0 -> 16) then one jump backward (16 -> 7) will get the bug home.

 
Constraints:

	1 <= forbidden.length <= 1000
	1 <= a, b, forbidden[i] <= 2000
	0 <= x <= 2000
	All the elements in forbidden are distinct.
	Position x is not forbidden.

 */
pub struct Solution {}

// submission codes start here

const NOT_VISITED: i32 = 0;
const BAK_VISITED: i32 = 1;
const FWD_VISITED: i32 = 2;
use std::collections::{VecDeque};
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
		let (a, b, x) = (a as usize, b as usize, x as usize);
		let mut ans = 0;
		let maxn = 6000; //x + a*2;
		let mut vis = vec![0; maxn + 1];
		forbidden.into_iter().for_each(|i| vis[i as usize] = FWD_VISITED);
		vis[0] = FWD_VISITED;
		let mut q = VecDeque::new();
		q.push_back((0, true));
		while !q.is_empty() {
			let mut n = q.len();
			while let Some((i, back)) = q.pop_front() {
				if i == x {
					return ans;
				}
				// jump forward
				if i + a <= maxn && vis[i + a] != FWD_VISITED { // allow back and then fwd
					vis[i + a] = FWD_VISITED;
					q.push_back((i + a, true));
				}
				// jump back
				if back && i >= b && vis[i - b] == NOT_VISITED { // if bak_visited, already visited; if fwd_visited, but now we can only to fwd after this back move, so repeating
					vis[i - b] = BAK_VISITED;
					q.push_back((i - b, false));
				}
				n -= 1;
				if n == 0 { break; }
			}
			ans += 1;
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
    }
}
