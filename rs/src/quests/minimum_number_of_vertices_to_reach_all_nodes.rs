/**
 * [1557] Minimum Number of Vertices to Reach All Nodes
 *
 * Given a directed acyclic graph, with n vertices numbered from 0 to n-1, and an array edges where edges[i] = [fromi, toi] represents a directed edge from node fromi to node toi.
Find the smallest set of vertices from which all nodes in the graph are reachable. It's guaranteed that a unique solution exists.
Notice that you can return the vertices in any order.
 
Example 1:

Input: n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]
Output: [0,3]
Explanation: It's not possible to reach all the nodes from a single vertex. From 0 we can reach [0,1,2,5]. From 3 we can reach [3,4,2,5]. So we output [0,3].
Example 2:

Input: n = 5, edges = [[0,1],[2,1],[3,1],[1,4],[2,4]]
Output: [0,2,3]
Explanation: Notice that vertices 0, 3 and 2 are not reachable from any other node, so we must include them. Also any of these vertices can reach nodes 1 and 4.

 
Constraints:

	2 <= n <= 10^5
	1 <= edges.length <= min(10^5, n * (n - 1) / 2)
	edges[i].length == 2
	0 <= fromi, toi < n
	All pairs (fromi, toi) are distinct.

 */
pub struct Solution {}

// submission codes start here

use std::{collections::{VecDeque, HashSet}, iter::FromIterator};
impl Solution {
    // trivial topsort
    // pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    //     let n = n as usize;
    //     let mut ans = vec![];
    //     let mut g = vec![HashSet::new(); n];
    //     let mut vis = (0..n).collect::<HashSet<_>>();
    //     for e in &edges {
    //         let (from ,to) = (e[0] as usize, e[1] as usize);
    //         g[to].insert(from);
    //         vis.remove(&from);
    //     }
    //     let mut q = VecDeque::from_iter(vis.iter().cloned());
    //     while let Some(curr) = q.pop_front() {
    //         // no in-edge to curr, it's a top-order root
    //         if g[curr].is_empty() {
    //             ans.push(curr as i32);
    //         } else {
    //             // remove in-edges
    //             for prev in g[curr].drain() {
    //                 if !vis.contains(&prev) {
    //                     vis.insert(prev);
    //                     q.push_back(prev);
    //                 }
    //             }
    //         }
    //     }
    //     ans
    // }

    // or simple: we just need to find vertices with in-degree = 0
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut in_degree = vec![false; n];
        edges.into_iter().for_each(|e| in_degree[e[1] as usize] = true);
        (0..n as i32).filter(|&i| in_degree[i as usize]).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v= Solution::find_smallest_set_of_vertices(6, vec_vec![[0,1],[0,2],[2,5],[3,4],[4,2]]);
        v.sort();
        assert_eq!(vec![0,3], v);
        let mut v= Solution::find_smallest_set_of_vertices(5, vec_vec![[0,1],[2,1],[3,1],[1,4],[2,4]]);
        v.sort();
        assert_eq!(vec![0,2,3], v);
    }
}
