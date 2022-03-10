/**
 * [547] Number of Provinces
 *
 * There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
A province is a group of directly or indirectly connected cities and no other cities outside of the group.
You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.
Return the total number of provinces.
 
Example 1:
Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
Output: 2

Example 2:
Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3

 
Constraints:

	1 <= n <= 200
	n == isConnected.length
	n == isConnected[i].length
	isConnected[i][j] is 1 or 0.
	isConnected[i][i] == 1
	isConnected[i][j] == isConnected[j][i]

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
            // merge rank smaller to bigger
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

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        // init union set
        let mut par = vec![0; n];
        let mut rank = vec![0; n];
        (0..n).for_each(|i| par[i] = i);
        // cons union set
        for i in 0..n-1 {
            for j in i+1..n {
                if is_connected[i][j] > 0 {
                    Self::union_merge(&mut par, &mut rank, i, j);
                }
            }
        }
        // count unions
        use std::collections::HashSet;
        (0..n).map(|x| Self::union_find(&mut par, x))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

// submission codes end

use std::iter::FromIterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(2, Solution::find_circle_num(vec_vec![[1,1,0],[1,1,0],[0,0,1]]));
        // assert_eq!(3, Solution::find_circle_num(vec_vec![[1,0,0],[0,1,0],[0,0,1]]));
        assert_eq!(1, Solution::find_circle_num(vec_vec![[1,0,0,1],[0,1,1,0],[0,1,1,1],[1,0,1,1]]))
    }
}
