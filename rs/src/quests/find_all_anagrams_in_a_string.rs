/**
 * [438] Find All Anagrams in a String
 *
 * Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 
Example 1:
Input: s = "cbaebabacd", p = "abc"
Output: [0,6]
Explanation:
The substring with start index = 0 is "cba", which is an anagram of "abc".
The substring with start index = 6 is "bac", which is an anagram of "abc".

Example 2:
Input: s = "abab", p = "ab"
Output: [0,1,2]
Explanation:
The substring with start index = 0 is "ab", which is an anagram of "ab".
The substring with start index = 1 is "ba", which is an anagram of "ab".
The substring with start index = 2 is "ab", which is an anagram of "ab".

 
Constraints:

	1 <= s.length, p.length <= 3 * 104
	s and p consist of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans = vec![];

        let n = p.len();
        let mut count = [0; 26];
        p.as_bytes().iter().for_each(|&c| count[(c - b'a') as usize] += 1);

        let mut win = [0; 26];
        let s = s.as_bytes();
        let mut i = 0;
        for j in 0..s.len() {
            win[(s[j] - b'a') as usize] += 1;
            if j - i + 1 < n { continue; }
            if j >= n {
                win[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }
            if win == count {
                ans.push((j + 1 - n) as i32);
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
        assert_eq!(vec![0,6], Solution::find_anagrams("cbaebabacd".into(), "abc".into()));
    }
}
