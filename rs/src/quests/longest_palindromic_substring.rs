/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

Example 1:

Input: "babad"
Output: "bab"
Note: "aba" is also a valid answer.


Example 2:

Input: "cbbd"
Output: "bb"

* How can we reuse a previously computed palindrome to compute a larger palindrome?
* If “aba” is a palindrome, is “xabax” and palindrome? Similarly is “xabay” a palindrome?
* Complexity based hint:
  If we use brute-force and check whether for every start and end position a substring is a palindrome
we have O(n^2) start - end pairs and O(n) palindromic checks. Can we reduce the time for palindromic
checks to O(1) by reusing some previous computation.

 */
pub struct Solution {}

// submission codes start here

use std::cmp;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return s;
        }
        let ss = s.as_bytes();
        let mut p = [0; 2002];
        let (mut id, mut mx) = (0usize, 0usize);
        for i in 0..2*(n-1)+1 {
            let mut chars: usize = if i < mx {
                cmp::min(p[id*2 - i], (mx - i)/2)
            } else { 0 };
            let (mut a, mut b) = ((i/2 - chars) as i32, ((i + 1)/2 + chars) as i32);
            while 0 <= a && b < n as i32 && ss[a as usize] == ss[b as usize] {
                    a -= 1;
                    b += 1;
                    chars += 1;
            }
            p[i] = chars;
            if b > 0 && (mx as i32) < (b*2 - 1) {
                id = i;
                mx = (b*2 - 1) as usize;
            }
        }
        let (mut mlen, mut clen, mut idx) = (0usize, 0usize, 0usize);
        for i in 0..2*(n-1)+1 {
            if i & 1 == 1 { // odd is #
                clen = p[i]*2;
                if clen > mlen {
                    mlen = clen;
                    idx = (i + 1 - p[i]*2)/2;
                }
            } else {
                clen = p[i]*2 - 1;
                if clen > mlen {
                    mlen = clen;
                    idx = (i + 2 - p[i]*2)/2;
                }
            }
        }
        s[idx..idx+mlen].to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let q = vec![
            "12212321",
            "babcbabcbaccba",
            "abaaba",
            "abababa",
            "abcbabcbabcba",
            "forgeeksskeegfor",
            "caba",
            "abacdfgdcaba",
            "abacdfgdcabba",
            "abacdedcaba",
            "bb",
            "",
        ];
        let a = vec![
            "12321",
            "abcbabcba",
            "abaaba",
            "abababa",
            "abcbabcbabcba",
            "geeksskeeg",
            "aba",
            "aba",
            "abba",
            "abacdedcaba",
            "bb",
            "",
        ];
        for (i, o) in q.into_iter().zip(a) {
            assert_eq!(Solution::longest_palindrome(i.to_owned()), o);
        }
    }
}
