/**
 * [1319] Number of Operations to Make Network Connected
 *
 * There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
 
Example 1:
Input: n = 4, connections = [[0,1],[0,2],[1,2]]
Output: 1
Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.

Example 2:
Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
Output: 2

Example 3:
Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
Output: -1
Explanation: There are not enough cables.

 
Constraints:

	1 <= n <= 105
	1 <= connections.length <= min(n * (n - 1) / 2, 105)
	connections[i].length == 2
	0 <= ai, bi < n
	ai != bi
	There are no repeated connections.
	No two computers are connected by more than one cable.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn union_find(par: &mut Vec<usize>, x: usize) -> usize {
        if par[x] == x {
            x
        } else {
            par[x] = Self::union_find(par, par[x]);
            par[x]
        }
    }

    fn union_merge(par: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let x = Self::union_find(par, x);
        let y = Self::union_find(par, y);
        if x != y {
            if rank[x] < rank[y] {
                par[x] = y;
            } else {
                par[y] = x;
                if rank[x] == rank[y] {
                    rank[x] += 1;
                }
            }
        }
    }

    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
		let n = n as usize;
		if connections.len() < n - 1 {
			return -1;
		}

		let mut par = vec![0; n];
		let mut rank = vec![0; n];
		(0..n).for_each(|i| par[i] = i);
		for c in connections {
			Self::union_merge(&mut par, &mut rank, c[0] as usize, c[1] as usize);
		}
        use std::collections::HashSet;
		let sets = (0..n).map(|x| Self::union_find(&mut par, x)).collect::<HashSet<_>>().len();
		(sets - 1) as i32
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
