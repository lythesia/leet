/**
 * [763] Partition Labels
 *
 * You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
Return a list of integers representing the size of these parts.
 
Example 1:
Input: s = "ababcbacadefegdehijhklij"
Output: [9,7,8]
Explanation:
The partition is "ababcbaca", "defegde", "hijhklij".
This is a partition so that each letter appears in at most one part.
A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.

Example 2:
Input: s = "eccbbbbdec"
Output: [10]

 
Constraints:

	1 <= s.length <= 500
	s consists of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let n = s.len();
        let mut ans = vec![];
        let mut last = 0;
        let mut i = 0;
        let mut k = 0;
        let s = s.as_bytes();
        while i < n {
            let mut h = HashMap::new();
            while i <= k {
                if !h.contains_key(&s[i]) {
                    // find next same with s[i] from tail
                    let mut j = n - 1;
                    while j > i && s[j] != s[i] { j -= 1; }
                    h.insert(s[i], j);
                    // enlarge range
                    k = k.max(j);
                }
                // try next i in range, until k
                i += 1;
            }
            // one range done
            ans.push((i - last) as i32);
            last = i;
            k = i;
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
        assert_eq!(vec![9,7,8], Solution::partition_labels("ababcbacadefegdehijhklij".into()));
        assert_eq!(vec![10], Solution::partition_labels("eccbbbbdec".into()));
    }
}
