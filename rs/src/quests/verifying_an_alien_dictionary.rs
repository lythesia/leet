/**
 * [953] Verifying an Alien Dictionary
 *
 * In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.
 
Example 1:
Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
Output: true
Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.

Example 2:
Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
Output: false
Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.

Example 3:
Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
Output: false
Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character which is less than any other character (More info).

 
Constraints:

	1 <= words.length <= 100
	1 <= words[i].length <= 20
	order.length == 26
	All characters in words[i] and order are English lowercase letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict = [0; 128];
        for (i, c) in order.chars().enumerate() {
            dict[c as usize] = i;
        }
        use std::cmp::Ordering;
        let cmp = |x: &String, y: &String| {
            let (mut x, mut y) = (x.as_bytes().iter().peekable(), y.as_bytes().iter().peekable());
            loop {
                match (x.peek(), y.peek()) {
                    (Some(&&i), Some(&&j)) => {
                        let ci = dict[i as usize];
                        let cj = dict[j as usize];
                        if ci > cj {
                            return Ordering::Greater;
                        } else if ci < cj {
                            return Ordering::Less;
                        }
                        x.next();
                        y.next();
                    },
                    _ => break,
                }
            }
            if x.count() > 0 {
                Ordering::Greater
            } else if y.count() > 0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        };
        (1..words.len()).all(|i| cmp(&words[i - 1], &words[i]) != Ordering::Greater)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::vec_string;

    use super::*;

    #[test]
    fn test() {
        // assert!(Solution::is_alien_sorted(vec_string!["hello", "leetcode"], "hlabcdefgijkmnopqrstuvwxyz".into()))
        assert!(!Solution::is_alien_sorted(vec_string!["aa", "a"], "abqwertyuioplkjhgfdszxcvnm".into()))
    }
}
