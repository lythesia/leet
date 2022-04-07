/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

 
Example 1:
Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

Example 2:
Input: digits = ""
Output: []

Example 3:
Input: digits = "2"
Output: ["a","b","c"]

 
Constraints:

	0 <= digits.length <= 4
	digits[i] is a digit in the range ['2', '9'].

 */
pub struct Solution {}

// submission codes start here

const DIAL: [&str; 10] = [
    "",
    "",
    "abc",
    "def",
    "ghi",
    "jkl",
    "mno",
    "pqrs",
    "tuv",
    "wxyz",
];
fn dfs(digits: &[u8], s: &mut String, v: &mut Vec<String>) {
    if digits.is_empty() {
        v.push(s.clone());
    } else {
        let d = digits[0] - b'0';
        for c in DIAL[d as usize].chars() {
            s.push(c);
            dfs(&digits[1..], s, v);
            s.pop();
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut v = vec![];
        if digits.is_empty() { return v; }
        let mut s = String::new();
        dfs(digits.as_bytes(), &mut s, &mut v);
        v
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
