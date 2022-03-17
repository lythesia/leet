/**
 * [785] Is Graph Bipartite?
 *
 * There is an undirected graph with n nodes, where each node is numbered between 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes that node u is adjacent to. More formally, for each v in graph[u], there is an undirected edge between node u and node v. The graph has the following properties:

	There are no self-edges (graph[u] does not contain u).
	There are no parallel edges (graph[u] does not contain duplicate values).
	If v is in graph[u], then u is in graph[v] (the graph is undirected).
	The graph may not be connected, meaning there may be two nodes u and v such that there is no path between them.

A graph is bipartite if the nodes can be partitioned into two independent sets A and B such that every edge in the graph connects a node in set A and a node in set B.
Return true if and only if it is bipartite.
 
Example 1:
Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
Output: false
Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
Example 2:
Input: graph = [[1,3],[0,2],[1,3],[0,2]]
Output: true
Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
 
Constraints:

	graph.length == n
	1 <= n <= 100
	0 <= graph[u].length < n
	0 <= graph[u][i] <= n - 1
	graph[u] does not contain u.
	All the values of graph[u] are unique.
	If graph[u] contains v, then graph[v] contains u.

 */
pub struct Solution {}

// submission codes start here

fn find(x: usize, par: &mut Vec<usize>) -> usize {
    if x == par[x] { x }
    else {
        par[x] = find(par[x], par);
        par[x]
    }
}
fn union(x: usize, y: usize, par: &mut Vec<usize>, rank: &mut Vec<usize>) {
    let px = find(x, par);
    let py = find(y, par);
    if rank[px] < rank[py] {
        par[px] = py;
    } else {
        if rank[px] == rank[py] {
            rank[px] += 1;
        }
        par[py] = px;
    }
}
impl Solution {
	// it means every adj (u,v) pair should be in different set
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
		let n = graph.len();
		let mut par = (0..n).collect::<Vec<_>>();
		let mut rank = vec![0; n];
		for (u, edges) in graph.into_iter().enumerate() {
			let mut last = u;
			// all v should not share same set with u
			// all v can be merged
			for v in edges {
				let v = v as usize;
				if find(u, &mut par) == find(v, &mut par) {
					return false;
				}
				if last != u {
					union(last, v, &mut par, &mut rank);
				}
				last = v;
			}
		}
		true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert!(!Solution::is_bipartite(vec_vec![[1,2,3],[0,2],[0,1,3],[0,2]]));
		assert!(Solution::is_bipartite(vec_vec![[1,3],[0,2],[1,3],[0,2]]));
    }
}
