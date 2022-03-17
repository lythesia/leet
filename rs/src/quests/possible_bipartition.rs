/**
 * [886] Possible Bipartition
 *
 * We want to split a group of n people (labeled from 1 to n) into two groups of any size. Each person may dislike some other people, and they should not go into the same group.
Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that the person labeled ai does not like the person labeled bi, return true if it is possible to split everyone into two groups in this way.
 
Example 1:
Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
Output: true
Explanation: group1 [1,4] and group2 [2,3].

Example 2:
Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
Output: false

Example 3:
Input: n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
Output: false

 
Constraints:

	1 <= n <= 2000
	0 <= dislikes.length <= 104
	dislikes[i].length == 2
	1 <= dislikes[i][j] <= n
	ai < bi
	All the pairs of dislikes are unique.

 */
pub struct Solution {}

// submission codes start here

fn union_find(x: usize, par: &mut Vec<usize>) -> usize {
    if x == par[x] { x }
    else {
        par[x] = union_find(par[x], par);
        par[x]
    }
}
fn union_merge(x: usize, y: usize, par: &mut Vec<usize>, rank: &mut Vec<usize>) {
    let px = union_find(x, par);
    let py = union_find(y, par);
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
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut par = (0..=n).collect::<Vec<_>>();
        let mut rank = vec![0; n + 1];
        let mut dislike_me = vec![0; n + 1]; // last person who dislike i
        for d in dislikes {
            let (x, y) = (d[0] as usize, d[1] as usize);
            if union_find(x, &mut par) == union_find(y, &mut par) {
                return false;
            }
            // merge two if they dislike same one
            if dislike_me[x] > 0 {
                union_merge(dislike_me[x], y, &mut par, &mut rank);
            }
            if dislike_me[y] > 0 {
                union_merge(x, dislike_me[y], &mut par, &mut rank);
            }
            dislike_me[x] = y;
            dislike_me[y] = x;
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
        // assert!(Solution::possible_bipartition(4, vec_vec![[1,2],[1,3],[2,4]]));
        assert!(!Solution::possible_bipartition(5, vec_vec![[1,2],[2,3],[3,4],[4,5],[1,5]]));
    }
}
