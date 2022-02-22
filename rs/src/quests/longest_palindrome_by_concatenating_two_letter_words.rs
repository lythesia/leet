/**
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 *
 * You are given an array of strings words. Each element of words consists of two lowercase English letters.
Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
A palindrome is a string that reads the same forward and backward.
 
Example 1:
Input: words = ["lc","cl","gg"]
Output: 6
Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
Note that "clgglc" is another longest palindrome that can be created.

Example 2:
Input: words = ["ab","ty","yt","lc","cl","ab"]
Output: 8
Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
Note that "lcyttycl" is another longest palindrome that can be created.

Example 3:
Input: words = ["cc","ll","xx"]
Output: 2
Explanation: One longest palindrome is "cc", of length 2.
Note that "ll" is another longest palindrome that can be created, and so is "xx".

 
Constraints:

	1 <= words.length <= 105
	words[i].length == 2
	words[i] consists of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /*
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut h = std::collections::HashMap::new();
        let mut p = std::collections::HashSet::new();
        let mut ans = 0;

        for s in &words {
            if Self::is_pali(s) {p.insert(s.to_owned());}
            *h.entry(s.to_owned()).or_insert(std::cell::RefCell::new(0)).borrow_mut() += 1;
        }

        // for all 2-letter word
        for (s, cell) in &h {
            // xx like: if count > 2, then prepend one and append one
            let mut c = cell.borrow_mut();
            if p.contains(s) {
                let d = (*c >> 1) << 1;
                ans += 2*d;
                if *c == d {
                    p.remove(s);
                }
            }
            // xy like: prepend xy and append yx, then reduce count
            else {
                let rs: String = s.chars().rev().collect();
                if let Some(cell2) = h.get(&rs) {
                    let mut d = cell2.borrow_mut();
                    let min = std::cmp::min(*c, *d);
                    ans += 4*min;
                    // reduce c, d
                    *c -= min;
                    *d -= min;
                }
            }
        }
        // if any xx rest, add to middle
        if !p.is_empty() { ans += 2; }

        ans
    }

    fn is_pali(s: &String) -> bool {
        let mut cs = s.chars();
        cs.next() == cs.next()
    }
    */

    // simple ver
    // since all letter in [a-z]
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut c = [[0; 26]; 26];
        let mut ans = 0;

        for s in &words {
            let mut chs = s.as_bytes();
            let (x, y) = ((chs[0] - 'a' as u8) as usize, (chs[1] - 'a' as u8) as usize);
            if c[y][x] > 0 {
                ans += 4;
                c[y][x] -= 1;
            } else {
                c[x][y] += 1;
            }
        }
        for i in 0..26 {
            if c[i][i] > 0 {
                ans += 2;
                break;
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
        assert_eq!(6, Solution::longest_palindrome(vec_string!["lc","cl","gg"]));
        assert_eq!(8, Solution::longest_palindrome(vec_string!["ab","ty","yt","lc","cl","ab"]));
        assert_eq!(2, Solution::longest_palindrome(vec_string!["cc", "ll", "xx"]));
    }
}
