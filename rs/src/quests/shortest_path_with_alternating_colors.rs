/**
 * [1129] Shortest Path with Alternating Colors
 *
 * You are given an integer n, the number of nodes in a directed graph where the nodes are labeled from 0 to n - 1. Each edge is red or blue in this graph, and there could be self-edges and parallel edges.
You are given two arrays redEdges and blueEdges where:

	redEdges[i] = [ai, bi] indicates that there is a directed red edge from node ai to node bi in the graph, and
	blueEdges[j] = [uj, vj] indicates that there is a directed blue edge from node uj to node vj in the graph.

Return an array answer of length n, where each answer[x] is the length of the shortest path from node 0 to node x such that the edge colors alternate along the path, or -1 if such a path does not exist.
 
Example 1:
Input: n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
Output: [0,1,-1]

Example 2:
Input: n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
Output: [0,1,-1]

 
Constraints:

	1 <= n <= 100
	0 <= redEdges.length, blueEdges.length <= 400
	redEdges[i].length == blueEdges[j].length == 2
	0 <= ai, bi, uj, vj < n

 */
pub struct Solution {}

// submission codes start here

const RED: u8 = 0b01;
const BLUE: u8 = 0b10;
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let add_edges = |edges: Vec<Vec<i32>>| {
            let mut g = vec![vec![]; n];
            for x in edges {
                let s = x[0] as usize;
                let t = x[1] as usize;
                g[s].push(t);
            }
            g
        };
        let red = add_edges(red_edges);
        let blue = add_edges(blue_edges);

        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::new();
        let mut vis = vec![0; n];
        // can start either with prev as red or blue
        q1.push_back((0, RED));
        q1.push_back((0, BLUE));
        vis[0] = RED | BLUE;
        let mut ans = vec![-1; n];
        let mut steps = 0;
        while !q1.is_empty() {
            while let Some((v, c)) = q1.pop_front() {
                if ans[v] == -1 {
                    ans[v] = steps;
                }
                let (next_color, next_graph) = if c == RED {
                    (BLUE, &blue)
                } else {
                    (RED, &red)
                };

                for &nextv in &next_graph[v] {
                    // not visited
                    if vis[nextv] & next_color == 0 {
                        vis[nextv] |= next_color;
                        q2.push_back((nextv, next_color))
                    }
                }
            }
            std::mem::swap(&mut q1, &mut q2);
            steps += 1;
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
        assert_eq!(vec![0,1,1], Solution::shortest_alternating_paths(3, vec_vec![[0,1],[0,2]], vec_vec![[1,0]]));
        assert_eq!(vec![0,1,2,3,7], Solution::shortest_alternating_paths(5, vec_vec![[0,1],[1,2],[2,3],[3,4]], vec_vec![[1,2],[2,3],[3,1]]));
    }
}
