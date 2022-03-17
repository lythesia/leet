/**
 * [1615] Maximal Network Rank
 *
 * There is an infrastructure of n cities with some number of roads connecting these cities. Each roads[i] = [ai, bi] indicates that there is a bidirectional road between cities ai and bi.
The network rank of two different cities is defined as the total number of directly connected roads to either city. If a road is directly connected to both cities, it is only counted once.
The maximal network rank of the infrastructure is the maximum network rank of all pairs of different cities.
Given the integer n and the array roads, return the maximal network rank of the entire infrastructure.
 
Example 1:

Input: n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
Output: 4
Explanation: The network rank of cities 0 and 1 is 4 as there are 4 roads that are connected to either 0 or 1. The road between 0 and 1 is only counted once.

Example 2:

Input: n = 5, roads = [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]
Output: 5
Explanation: There are 5 roads that are connected to cities 1 or 2.

Example 3:
Input: n = 8, roads = [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]]
Output: 5
Explanation: The network rank of 2 and 5 is 5. Notice that all the cities do not have to be connected.

 
Constraints:

	2 <= n <= 100
	0 <= roads.length <= n * (n - 1) / 2
	roads[i].length == 2
	0 <= ai, bi <= n-1
	ai != bi
	Each pair of cities has at most one road connecting them.

 */
pub struct Solution {}

// submission codes start here

use std::cmp::max;
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![false; n]; n];
        for r in roads {
            let (s, e) = (r[0] as usize, r[1] as usize);
            g[s][e] = true;
            g[e][s] = true;
        }
        let mut ans = 0;
        for s in 0..n-1 {
            for e in s+1..n {
                // pair (s, e)
                let mut rank = g[s].iter().filter(|&&b| b).count() + g[e].iter().filter(|&&b| b).count();
                if g[s][e] { rank -= 1; }
                ans = max(ans, rank as i32);
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
        assert_eq!(5, Solution::maximal_network_rank(5, vec_vec![[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]));
    }
}
