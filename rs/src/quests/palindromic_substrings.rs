/**
 * [647] Palindromic Substrings
 *
 * Given a string s, return the number of palindromic substrings in it.
A string is a palindrome when it reads the same backward as forward.
A substring is a contiguous sequence of characters within the string.
 
Example 1:
Input: s = "abc"
Output: 3
Explanation: Three palindromic strings: "a", "b", "c".

Example 2:
Input: s = "aaa"
Output: 6
Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".

 
Constraints:

	1 <= s.length <= 1000
	s consists of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /*
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0i32;
        let n = s.len();
        let b = s.as_bytes();
        let mut p = vec![vec![false; n+1]; n+1]; // p[i][j]: s[i-1 ..= j-1] is pali
        for i in 0..n {
            p[i+1][i+1] = true;
        }
        // p[i][j](i<=j) = p[i+1][j-1] && s[i-1]==s[j-1] ? true : false
        // start from bottom-left, i<-n..0 j<-i..n
        for i in (1..n).rev() {   // [n-1 to 1]
            p[i][i+1] = b[i-1] == b[i]; // len=2
            if p[i][i+1] {ans += 1;}
            for j in i+2..n+1 { // [i+2 to n]
                p[i][j] = p[i+1][j-1] && b[i-1] == b[j-1];
                if p[i][j] {ans += 1;}
            }
        }
        ans + n as i32
    }
    */

    // simple ver
    // odd pali: for each one-letter-str: extend left and right if succeed
    // event: for each two-letter-str: the same
    pub fn count_substrings(s: String) -> i32 {
        let b = s.as_bytes();
        // let count = |i, j, s: &[u8]| {
        //     if j == s.len() || s[i] != s[j] { 0 }
        //     else {
        //         let (mut i, mut j) = (i, j);
        //         let mut ans = 1;
        //         while i > 0 && j < s.len()-1 && s[i-1] == s[j+1] {
        //             i -= 1;
        //             j += 1;
        //             ans += 1;
        //         }
        //         ans
        //     }
        // };
        // (0..s.len()).map(|i| count(i, i, b) + count(i, i + 1, b)).sum()

        // or simplest
        let cnt = |i: usize, j: usize| {
            ((0..=i).rev()).zip(j..b.len())
            .try_fold(0, |acc, (i, j)| if b[i] == b[j] {Ok(acc + 1)} else {Err(acc)})
            .unwrap_or_else(|acc| acc)
        };
        (0..s.len()).map(|i| cnt(i, i) + cnt(i, i + 1)).sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::count_substrings("abc".into()));
        assert_eq!(6, Solution::count_substrings("aaa".into()));
        assert_eq!(10, Solution::count_substrings("aaaa".into()));
    }
}
