/**
 * [127] Word Ladder
 *
 * A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

	Every adjacent pair of words differs by a single letter.
	Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
	sk == endWord

Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
 
Example 1:
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5
Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.

Example 2:
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
Output: 0
Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.

 
Constraints:

	1 <= beginWord.length <= 10
	endWord.length == beginWord.length
	1 <= wordList.length <= 5000
	wordList[i].length == beginWord.length
	beginWord, endWord, and wordList[i] consist of lowercase English letters.
	beginWord != endWord
	All the words in wordList are unique.

 */
pub struct Solution {}

// submission codes start here

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
		let (s, e) = (begin_word.into_bytes(), end_word.into_bytes());
		let words = word_list.into_iter().map(String::into_bytes).collect::<HashSet<_>>();
		if !words.contains(&e) { return 0; }
		let mut vis = HashSet::new();
		let mut q = VecDeque::new();
		vis.insert(s.clone());
		q.push_back(s);
		let mut ans = 1;
		while !q.is_empty() {
			let mut n = q.len();
			while let Some(mut w) = q.pop_front() {
				if w == e {
					return ans;
				}
				for i in 0..w.len() {
					let ch = w[i];
					for v in (b'a'..=b'z').filter(|&x| x != ch) {
						w[i] = v;
						if words.contains(&w) && !vis.contains(&w) {
							vis.insert(w.clone());
							q.push_back(w.clone());
						}
					}
					w[i] = ch;
				}
				n -= 1;
				if n == 0 { break; }
			}
			ans += 1;
		}
		0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert_eq!(5, Solution::ladder_length("hit".into(), "cog".into(), vec_string!["hot","dot","dog","lot","log","cog"]));
		assert_eq!(0, Solution::ladder_length("hit".into(), "cog".into(), vec_string!["hot","dot","dog","lot","log"]));
		assert_eq!(10, Solution::ladder_length("ymain".into(), "oecij".into(), vec_string!["ymann","yycrj","oecij","ymcnj","yzcrj","yycij","xecij","yecij","ymanj","yzcnj","ymain"]));
    }
}
