/**
 * [20] Valid Parentheses
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
An input string is valid if:

	Open brackets must be closed by the same type of brackets.
	Open brackets must be closed in the correct order.

 
Example 1:
Input: s = "()"
Output: true

Example 2:
Input: s = "()[]{}"
Output: true

Example 3:
Input: s = "(]"
Output: false

 
Constraints:

	1 <= s.length <= 104
	s consists of parentheses only '()[]{}'.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut pair = [' '; 128];
        pair['(' as usize] = ')';
        pair['[' as usize] = ']';
        pair['{' as usize] = '}';
        for c in s.chars() {
            match c {
                '('|'['|'{' => stack.push(c),
                ')'|']'|'}' => {
                    if let Some(&top) = stack.last() {
                        if pair[top as usize] == c {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else { return false; }
                },
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_valid("()[]{}".into()));
    }
}
