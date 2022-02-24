/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".
 
Example 1:
Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:
Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

 
Constraints:

	1 <= strs.length <= 200
	0 <= strs[i].length <= 200
	strs[i] consists of only lower-case English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();
        let mut ss = strs.into_iter().map(|s| s.into_bytes());
        let mut s0 = ss.next().unwrap();
        for (i, c) in s0.into_iter().enumerate() {
            if ss.clone().all(|it| i < it.len() && it[i] == c) { ans.push(c as char); }
            else { break; }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("fl".to_owned(), Solution::longest_common_prefix(vec_string!["flower","flow","flight"]));
        assert_eq!("".to_owned(), Solution::longest_common_prefix(vec_string!["dog","racecar","car"]));
    }
}
