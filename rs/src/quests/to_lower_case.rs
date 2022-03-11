/**
 * [709] To Lower Case
 *
 * Given a string s, return the string after replacing every uppercase letter with the same lowercase letter.
 
Example 1:
Input: s = "Hello"
Output: "hello"

Example 2:
Input: s = "here"
Output: "here"

Example 3:
Input: s = "LOVELY"
Output: "lovely"

 
Constraints:

	1 <= s.length <= 100
	s consists of printable ASCII characters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_ascii_lowercase()
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
