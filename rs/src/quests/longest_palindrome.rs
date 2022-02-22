/**
 * [409] Longest Palindrome
 *
 * Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
 
Example 1:
Input: s = "abccccdd"
Output: 7
Explanation:
One longest palindrome that can be built is "dccaccd", whose length is 7.

Example 2:
Input: s = "a"
Output: 1

Example 3:
Input: s = "bb"
Output: 2

 
Constraints:

	1 <= s.length <= 2000
	s consists of lowercase and/or uppercase English letters only.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // just count each letter, ans = s.len() - number_of_odd_counters + 1(if odd > 0)
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = [0; 128];
        for &b in s.as_bytes() {
            cnt[b as usize] += 1;
        }

        let mut odd = 0;
        for c in cnt {
            odd += (c & 1);
        }

        (if odd > 0 {
            s.len() - odd + 1
        } else {
            s.len()
        }) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, Solution::longest_palindrome("abccccdd".into()));
        assert_eq!(1, Solution::longest_palindrome("a".into()));
        assert_eq!(2, Solution::longest_palindrome("aa".into()));
    }
}
