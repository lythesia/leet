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
    /*
    Manachar:
    S: insert # between each char
    P: longest len of a Pali extend to left(or right), with S[i] included
    given:
       [  1   2   2   1   2   3   2   1  ]
    S: [# 1 # 2 # 2 # 1 # 2 # 3 # 2 # 1 #]
    P: [1 2 1 2 5 2 1 4 1 2 1 6 1 2 1 2 1], thus P[i] - 1 = len of Pali(cetered at i)
    (assume n = actual len of Pali, all(left to right) = n + (n + 1), P[i] = n + 1, then n = P[i] - 1)
    mx = id + P[i]
    mx: i之前的最长P: Pali的最远的点
    id: P的中心

    一般情况:
    A. i >= mx:
        j   mx'     id     mx   i        <- i and j are symmetirc to id, so are mx and mx'
    ----+----+------+------+----+--------
    P[i]只能通过最朴素的Pali定义的方式来计算

    可推导情况:
    B. i < mx:

       mx'   j     id      i   mx        <- i and j are symmetirc to id, so are mx and mx'
    ----+----+------+------+----+--------

    extending mx until S end
    [mx' to mx]是一个以id为中心的最长Pali, 那么以i为中心的, 且边界不超过mx的Pali一定在[mx' to id]中

    1. if mx - i > P[j]: P[i] = P[j]
    2. if mx - i <= P[j]:
        P[i] = mx - i, 然后再进行朴素计算, 因为mx'左边的已经不包括在P[j]中了
    (朴素计算: while A[i - P[i]] == A[i + P[i]]: P[i]++)
    */
    pub fn longest_pali(s: String) -> String {
        let n = s.len();
        if n == 0 { return s; }
        let nn = 2*n+1;
        let mut ss = String::with_capacity(2*(n + 1));
        ss.push('^');
        for c in s.chars() {
            ss.push('#');
            ss.push(c);
        }
        ss.push_str("#$");

        let mut p = [0; 2002];
        let (mut id, mut mx) = (0usize, 0usize);
        let (mut s, mut e, mut mmax) = (0usize, 0usize, 0usize);
        for i in 1..nn {
            let mut pi = 
            // B
            if i < mx {
                let j = id*2 - i;
                // B.1
                if mx - i + 1 > p[j] { p[j] }
                // B.2
                else { mx - i + 1 }
            }
            // A
            else {
                1
            };
            // trivial compute
            let slice = ss.as_bytes();
            while slice[i - pi] == slice[i + pi] { pi += 1; }
            p[i] = pi;
            // update id and mx
            if i + p[i] - 1 > mx {
                mx = i + p[i] - 1;
                id = i;
            }
            // update index and mmax
            if p[i] > mmax {
                mmax = p[i];
                s = i - mmax + 1;
                e = i + mmax - 1;
            }
        }
        let mut ans = String::with_capacity(mmax - 1);
        for c in ss[s..e].chars() {
            if c != '#' { ans.push(c); }
        }

        ans
    }
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
            assert_eq!(Solution::longest_pali(i.to_owned()), o);
        }
    }
}
