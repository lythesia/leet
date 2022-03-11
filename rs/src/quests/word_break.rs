/**
 * [139] Word Break
 *
 * Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
Note that the same word in the dictionary may be reused multiple times in the segmentation.
 
Example 1:
Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: Return true because "leetcode" can be segmented as "leet code".

Example 2:
Input: s = "applepenapple", wordDict = ["apple","pen"]
Output: true
Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
Note that you are allowed to reuse a dictionary word.

Example 3:
Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
Output: false

 
Constraints:

	1 <= s.length <= 300
	1 <= wordDict.length <= 1000
	1 <= wordDict[i].length <= 20
	s and wordDict[i] consist of only lowercase English letters.
	All the strings of wordDict are unique.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
struct Trie {
    end: bool,
    children: HashMap<u8, Box<Trie>>,
}
impl Trie {
    fn new() -> Self {
        Self {
            end: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for &ch in word.as_bytes() {
            curr = curr.children.entry(ch).or_insert(Box::new(Trie::new()));
        }
        curr.end = true;
    }
}

impl Solution {
    // this can be memorized
    fn dfs(par: &Trie, s: &[u8], tree: &Trie, memo: &mut HashMap<(*const Trie, *const u8), bool>) -> bool {
        let k = (par as *const Trie, s.as_ptr());
        if let Some(&ret) = memo.get(&k) {
            return ret;
        }

        let ret = if s.is_empty() { par.end }
        else {
            let ch = s[0];
            if let Some(x) = par.children.get(&ch) {
                if x.end {
                    // break early or try descend
                    Self::dfs(tree, &s[1..], tree, memo) || Self::dfs(x, &s[1..], tree, memo)
                } else {
                    // only to descend now
                    Self::dfs(x, &s[1..], tree, memo)
                }
            } else if par.end {
                Self::dfs(tree, s, tree, memo)
            } else {
                false
            }
        };
        memo.insert(k, ret);
        ret
    }

    // dp[i] = valid at string of length i
    // dp[0] = true
    // dp[i + word[j].len()] = true, if s.find(word[j], from_i) == i && dp[i]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut tree = Trie::new();
        for word in word_dict {
            tree.insert(word);
        }
        let mut memo = HashMap::new();
        Self::dfs(&tree, s.as_bytes(), &tree, &mut memo)

        // let n = s.len();
        // let mut dp = vec![false; n + 1];
        // dp[0] = true;
        // for i in 0..n {
        //     if dp[i] {
        //         for w in &word_dict {
        //             if s[i..].starts_with(w) {
        //                 dp[i + w.len()] = true;
        //             }
        //         }
        //     }
        // }
        // dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::word_break("leetcode".into(), vec_string!["leet","code"]));
        assert!(Solution::word_break("applepenapple".into(), vec_string!["apple","pen"]));
        assert!(!Solution::word_break("catsandog".into(), vec_string!["cats","dog","sand","and","cat"]));
        assert!(Solution::word_break("abcd".into(), vec_string!["a","abc","b","cd"]));
        assert!(!Solution::word_break("aaaaaaa".into(), vec_string!["aaaa","aa"]));
    }
}
