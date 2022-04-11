/**
 * [1249] Minimum Remove to Make Valid Parentheses
 *
 * Given a string s of '(' , ')' and lowercase English characters.
Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
Formally, a parentheses string is valid if and only if:

	It is the empty string, contains only lowercase characters, or
	It can be written as AB (A concatenated with B), where A and B are valid strings, or
	It can be written as (A), where A is a valid string.

 
Example 1:
Input: s = "lee(t(c)o)de)"
Output: "lee(t(c)o)de"
Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.

Example 2:
Input: s = "a)b(c)d"
Output: "ab(c)d"

Example 3:
Input: s = "))(("
Output: ""
Explanation: An empty string is also valid.

 
Constraints:

	1 <= s.length <= 105
	s[i] is either'(' , ')', or lowercase English letter.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut ans = String::new();
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(ans.len());
                    ans.push(c);
                },
                ')' => {
                    if let Some(top) = stack.pop() {
                        ans.push(c);
                    }
                },
                _ => ans.push(c),
            }
        }
        let mut v = vec![];
        let mut rest = ans.as_str();
        // [0, i)
        let mut last = 0;
        for i in stack {
            // split
            let (x, y) = rest.split_at(i - last);
            v.push(x);
            if y.is_empty() { break; }
            else { rest = &y[1..]; }
            last = i + 1;
        }
        if !rest.is_empty() { v.push(rest); }
        v.join("")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", Solution::min_remove_to_make_valid("aa))b((".into()));
    }
}
