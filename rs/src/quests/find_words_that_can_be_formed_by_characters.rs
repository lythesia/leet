/**
 * [1160] Find Words That Can Be Formed by Characters
 *
 * You are given an array of strings words and a string chars.
A string is good if it can be formed by characters from chars (each character can only be used once).
Return the sum of lengths of all good strings in words.
 
Example 1:
Input: words = ["cat","bt","hat","tree"], chars = "atach"
Output: 6
Explanation: The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.

Example 2:
Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
Output: 10
Explanation: The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.

 
Constraints:

	1 <= words.length <= 1000
	1 <= words[i].length, chars.length <= 100
	words[i] and chars consist of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        // let mut ans = 0;
        // let mut c = [0; 26];
        // for ch in chars.chars() {
        //     c[(ch as i32 - 'a' as i32) as usize] += 1;
        // }
        // 'outer: for w in words {
        //     let mut h = std::collections::HashMap::new();
        //     let n = w.len() as i32;
        //     for ch in w.chars() {
        //         *h.entry(ch).or_insert(0) += 1;
        //         if h[&ch] > c[(ch as i32 - 'a' as i32) as usize] { continue 'outer; }
        //     }
        //     ans += n;
        // }
        // ans

        let counter = |s: &String| {
            let mut c = [0; 26];
            s.as_bytes().iter().for_each(|b| c[(b - b'a') as usize] += 1);
            c
        };
        let cnt = counter(&chars);
        words.into_iter()
            .filter(|w| counter(w).iter().enumerate().all(|(i, n)| cnt[i] >= *n))
            .map(|w| w.len() as i32)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, Solution::count_characters(vec_string!["cat","bt","hat","tree"], "atach".into()));
        assert_eq!(10, Solution::count_characters(vec_string!["hello","world","leetcode"], "welldonehoneyr".into()));
    }
}
