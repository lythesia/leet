/**
 * [752] Open the Lock
 *
 * You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'. The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'. Each move consists of turning one wheel one slot.
The lock initially starts at '0000', a string representing the state of the 4 wheels.
You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.
Given a target representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.
 
Example 1:
Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
Output: 6
Explanation: 
A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
because the wheels of the lock become stuck after the display becomes the dead end "0102".

Example 2:
Input: deadends = ["8888"], target = "0009"
Output: 1
Explanation: We can turn the last wheel in reverse to move from "0000" -> "0009".

Example 3:
Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
Output: -1
Explanation: We cannot reach the target without getting stuck.

 
Constraints:

	1 <= deadends.length <= 500
	deadends[i].length == 4
	target.length == 4
	target will not be in the list deadends.
	target and deadends[i] consist of digits only.

 */
pub struct Solution {}

// submission codes start here

const DIGIT: [usize; 4] = [1, 10, 100, 1000];
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut dead = [false; 10000];
        deadends.into_iter().for_each(|s| dead[s.parse::<usize>().unwrap()] = true);
        if dead[0] { return -1; }
        let target = target.parse::<usize>().unwrap();
        let mut q = VecDeque::new();
        let mut ans = 0;
        q.push_back(0);
        dead[0] = true;

        let digit_at = |x: usize, i: usize| (x / DIGIT[i]) % 10;
        while !q.is_empty() {
            let mut n = q.len();
            while let Some(code) = q.pop_front() {
                if code == target {
                    return ans;
                }
                for i in 0..4 {
                    // up
                    let next = if digit_at(code, i) == 9 {
                        code - 9*DIGIT[i]
                    } else {
                        code + DIGIT[i]
                    };
                    if !dead[next] {
                        q.push_back(next);
                        dead[next] = true;
                    }
                    // down
                    let next = if digit_at(code, i) == 0 {
                        code + 9*DIGIT[i]
                    } else {
                        code - DIGIT[i]
                    };
                    if !dead[next] {
                        q.push_back(next);
                        dead[next] = true;
                    }
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
        assert_eq!(6, Solution::open_lock(vec_string!["0201","0101","0102","1212","2002"], "0202".into()));
        assert_eq!(1, Solution::open_lock(vec_string!["8888"], "0009".into()));
        assert_eq!(-1, Solution::open_lock(vec_string!["8887","8889","8878","8898","8788","8988","7888","9888"], "8888".into()));
    }
}
