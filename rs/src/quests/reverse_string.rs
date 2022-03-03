/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters s.
You must do this by modifying the input array in-place with O(1) extra memory.
 
Example 1:
Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]
Example 2:
Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]
 
Constraints:

	1 <= s.length <= 105
	s[i] is a printable ascii character.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // s.reverse()
        let n = s.len();
        (0..n/2).zip((n-n/2..n).rev()).for_each(|(i, j)| s.swap(i, j))
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
