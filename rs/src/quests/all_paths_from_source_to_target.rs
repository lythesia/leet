/**
 * [797] All Paths From Source to Target
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1, find all possible paths from node 0 to node n - 1 and return them in any order.
The graph is given as follows: graph[i] is a list of all nodes you can visit from node i (i.e., there is a directed edge from node i to node graph[i][j]).
 
Example 1:
Input: graph = [[1,2],[3],[3],[]]
Output: [[0,1,3],[0,2,3]]
Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.

Example 2:
Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]

 
Constraints:

	n == graph.length
	2 <= n <= 15
	0 <= graph[i][j] < n
	graph[i][j] != i (i.e., there will be no self-loops).
	All the elements of graph[i] are unique.
	The input graph is guaranteed to be a DAG.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn dfs(g: &Vec<Vec<i32>>, s: usize, n: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if s == n - 1 {
            ans.push(path.clone());
        } else {
            for &e in &g[s] {
                path.push(e);
                Self::dfs(g, e as usize, n, path, ans);
                path.pop();
            }
        }
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len();
        let mut ans = vec![];
        let mut path = vec![0];
        Self::dfs(&graph, 0, n, &mut path, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec_vec![[0,1,3],[0,2,3]], Solution::all_paths_source_target(vec_vec![[1,2],[3],[3],[]]));
        assert_eq!(vec_vec![[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]], Solution::all_paths_source_target(vec_vec![[4,3,1],[3,2,4],[3],[4],[]]));
    }
}
