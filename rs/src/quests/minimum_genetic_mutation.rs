/**
 * [433] Minimum Genetic Mutation
 *
 * A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.
Suppose we need to investigate a mutation from a gene string start to a gene string end where one mutation is defined as one single character changed in the gene string.

	For example, "AACCGGTT" --> "AACCGGTA" is one mutation.

There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.
Given the two gene strings start and end and the gene bank bank, return the minimum number of mutations needed to mutate from start to end. If there is no such a mutation, return -1.
Note that the starting point is assumed to be valid, so it might not be included in the bank.
 
Example 1:
Input: start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
Output: 1

Example 2:
Input: start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
Output: 2

Example 3:
Input: start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
Output: 3

 
Constraints:

	start.length == 8
	end.length == 8
	0 <= bank.length <= 10
	bank[i].length == 8
	start, end, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].

 */
pub struct Solution {}

// submission codes start here

const DIR: [u8; 4] = [b'A', b'C', b'G', b'T'];
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let bank = bank.into_iter().map(String::into_bytes).collect::<HashSet<_>>();
        let mut vis = HashSet::new();
        let mut ans = 0;
        let e = end.into_bytes();
        let len = e.len();
        let mut q = VecDeque::new();
        let s = start.into_bytes();
        q.push_back(s.clone());
        vis.insert(s);
        while !q.is_empty() {
            let mut n = q.len();
            while let Some(mut g) = q.pop_front() {
                if g == e {
                    return ans;
                }
                for i in 0..len {
                    let gi = g[i];
                    // mutate
                    for &v in DIR.iter().filter(|&&x| x != gi) {
                        g[i] = v;
                        if bank.contains(&g) && !vis.contains(&g) {
                            q.push_back(g.clone());
                            vis.insert(g.clone());
                        }
                    }
                    // restore
                    g[i] = gi;
                }
                n -= 1;
                if n == 0 { break; }
            }
            ans += 1;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::min_mutation("AACCGGTT".into(), "AACCGGTA".into(), vec_string!["AACCGGTA"]));
        assert_eq!(2, Solution::min_mutation("AACCGGTT".into(), "AAACGGTA".into(), vec_string!["AACCGGTA","AACCGCTA","AAACGGTA"]));
        assert_eq!(3, Solution::min_mutation("AAAAACCC".into(), "AACCCCCC".into(), vec_string!["AAAACCCC","AAACCCCC","AACCCCCC"]));
    }
}
