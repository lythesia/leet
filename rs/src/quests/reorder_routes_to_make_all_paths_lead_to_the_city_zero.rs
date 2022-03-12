/**
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 *
 * There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
It's guaranteed that each city can reach city 0 after reorder.
 
Example 1:
Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
Output: 3
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 2:
Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
Output: 2
Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).

Example 3:
Input: n = 3, connections = [[1,0],[2,0]]
Output: 0

 
Constraints:

	2 <= n <= 5 * 104
	connections.length == n - 1
	connections[i].length == 2
	0 <= ai, bi <= n - 1
	ai != bi

 */
pub struct Solution {}

// submission codes start here
impl Solution {
    // dfs from e = 0 backwards
    fn dfs(e: usize, laste: usize, g: &Vec<Vec<usize>>, r: &Vec<Vec<usize>>) -> i32 {
        let mut n = 0;
        for &t in &g[e] {
            if t != laste {
                n += 1 + Self::dfs(t, e, g, r);
            }
        }
        for &t in &r[e] {
            if t != laste {
                n += Self::dfs(t, e, g, r);
            }
        }
        n
    }

    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut r = g.clone();
        for c in connections {
            let s = c[0] as usize;
            let t = c[1] as usize;
            // s to t
            g[s].push(t);
            // t from s
            r[t].push(s);
        }
        Self::dfs(0, usize::MAX, &g, &r)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::min_reorder(6, vec_vec![[0,1],[1,3],[2,3],[4,0],[4,5]]));
        assert_eq!(2, Solution::min_reorder(5, vec_vec![[1,0],[1,2],[3,2],[3,4]]));
        assert_eq!(0, Solution::min_reorder(3, vec_vec![[1,0],[2,0]]));
    }
}
