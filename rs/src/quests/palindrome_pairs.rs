/**
 * [336] Palindrome Pairs
 *
 * Given a list of unique words, return all the pairs of the distinct indices (i, j) in the given list, so that the concatenation of the two words words[i] + words[j] is a palindrome.
 
Example 1:
Input: words = ["abcd","dcba","lls","s","sssll"]
Output: [[0,1],[1,0],[3,2],[2,4]]
Explanation: The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]

Example 2:
Input: words = ["bat","tab","cat"]
Output: [[0,1],[1,0]]
Explanation: The palindromes are ["battab","tabbat"]

Example 3:
Input: words = ["a",""]
Output: [[0,1],[1,0]]

 
Constraints:

	1 <= words.length <= 5000
	0 <= words[i].length <= 300
	words[i] consists of lower-case English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn is_pali(s: &str) -> bool {
        s.chars().zip(s.chars().rev()).all(|(x, y)| x == y)
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut pali = std::collections::HashSet::new();
        let mut revs = std::collections::HashMap::new();
        let mut ans = Vec::new();
        
        for (i, s) in words.iter().enumerate() {
            let rs: String = s.chars().rev().collect();
            if &rs == s { pali.insert(s.to_owned()); }
            revs.insert(rs, i);
        }

        for (i, s) in words.iter().enumerate() {
            // deal empty and s: pali, (pali, "")
            if let Some(id) = revs.get("") {
                if *id != i && pali.contains(s) {
                    ans.push(vec![i as i32, *id as i32]);
                }
            }

            for j in 1..=s.len() {
                let left = &s[0..j]; // left from 1 to all-of-self
                let right = &s[j..]; // right from head-char-off to empty
                // 1. left + right(pali) + left.rev => (i, j)
                if let Some(id) = revs.get(left) {
                    if *id != i && Self::is_pali(right) {
                        ans.push(vec![i as i32, *id as i32]);
                    }
                }
                // 2. right.rev + left(pali) + right => (j, i), includes ("", pali)
                if let Some(id) = revs.get(right) {
                    if *id != i && Self::is_pali(left) {
                        ans.push(vec![*id as i32, i as i32]);
                    }
                }
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
        assert_eq!(vec_vec![[0,1],[1,0],[3,2],[2,4]], Solution::palindrome_pairs(vec_string!["abcd","dcba","lls","s","sssll"]));
        assert_eq!(vec_vec![[0,1],[1,0]], Solution::palindrome_pairs(vec_string!["bat","tab","cat"]));
        assert_eq!(vec_vec![[0,1],[1,0]], Solution::palindrome_pairs(vec_string!["a",""]));
    }
}
