/**
 * [72] Edit Distance
 *
 * Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
You have the following three operations permitted on a word:

	Insert a character
	Delete a character
	Replace a character

 
Example 1:
Input: word1 = "horse", word2 = "ros"
Output: 3
Explanation: 
horse -> rorse (replace 'h' with 'r')
rorse -> rose (remove 'r')
rose -> ros (remove 'e')

Example 2:
Input: word1 = "intention", word2 = "execution"
Output: 5
Explanation: 
intention -> inention (remove 't')
inention -> enention (replace 'i' with 'e')
enention -> exention (replace 'n' with 'x')
exention -> exection (replace 'n' with 'c')
exection -> execution (insert 'u')

 
Constraints:

	0 <= word1.length, word2.length <= 500
	word1 and word2 consist of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

use std::cmp::min;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        for i in 0..=m {
            for j in 0..=n {
                dp[i][j] = match (i, j) {
                    (0, _) => j,
                    (_, 0) => i,
                    _ => {
                        if w1[i - 1] == w2[j - 1] {
                            dp[i - 1][j - 1]
                        } else {
                            min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1
                        }
                    },
                };
            }
        }
        dp[m][n] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::min_distance("horse".into(), "ros".into()));
        assert_eq!(5, Solution::min_distance("intention".into(), "execution".into()));
    }
}
