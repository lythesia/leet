/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.


Example 1:

Input: "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.



Example 2:

Input: "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.



Example 3:

Input: "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
             Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 */
use std::cmp;
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut r = String::new();
        let mut ans = 0;
        for c in s.chars() {
            match r.find(c) {
                Some(i) => { r.drain(..i+1); r.push(c); println!("{} => {}", c, r); },
                _ => { r.push(c); println!("{} => {}", c, r); ans = cmp::max(ans, r.len() as i32); }
            }
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
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("aabaab!bb".to_string()), 2);
    }
}
