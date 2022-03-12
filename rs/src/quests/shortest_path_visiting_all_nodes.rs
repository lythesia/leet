/**
 * [847] Shortest Path Visiting All Nodes
 *
 * You have an undirected, connected graph of n nodes labeled from 0 to n - 1. You are given an array graph where graph[i] is a list of all the nodes connected with node i by an edge.
Return the length of the shortest path that visits every node. You may start and stop at any node, you may revisit nodes multiple times, and you may reuse edges.
 
Example 1:
Input: graph = [[1,2,3],[0],[0],[0]]
Output: 4
Explanation: One possible path is [1,0,2,0,3]

Example 2:
Input: graph = [[1],[0,2,4],[1,3,4],[2],[1,2]]
Output: 4
Explanation: One possible path is [0,1,4,2,3]

 
Constraints:

	n == graph.length
	1 <= n <= 12
	0 <= graph[i].length < n
	graph[i] does not contain i.
	If graph[a] contains b, then graph[b] contains a.
	The input graph is always connected.

 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let vis_all = (1<<n) - 1;
        // (curr_node, vis_bit_map)
        let mut vis = vec![vec![false; 1<<n]; n];
        let mut q = (1..n).map(|i| {
            let vis_bit = 1<<i;
            vis[i][vis_bit] = true;
            (i, vis_bit)
        }).collect::<VecDeque<_>>();

        let mut ans = 0;
        while !q.is_empty() {
            let mut len = q.len();
            while let Some((node, state)) = q.pop_front() {
                if state == vis_all {
                    return ans;
                }
                for &next in &graph[node] {
                    let next = next as usize;
                    let next_state = state | (1<<next);
                    if !vis[next][next_state] {
                        vis[next][next_state] = true;
                        q.push_back((next, next_state));
                    }
                }
                len -= 1;
                if len == 0 { break; }
            }
            ans += 1;
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
    }
}
