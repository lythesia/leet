/**
 * [997] Find the Town Judge
 *
 * In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
If the town judge exists, then:

	The town judge trusts nobody.
	Everybody (except for the town judge) trusts the town judge.
	There is exactly one person that satisfies properties 1 and 2.

You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi.
Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.
 
Example 1:
Input: n = 2, trust = [[1,2]]
Output: 2

Example 2:
Input: n = 3, trust = [[1,3],[2,3]]
Output: 3

Example 3:
Input: n = 3, trust = [[1,3],[2,3],[3,1]]
Output: -1

 
Constraints:

	1 <= n <= 1000
	0 <= trust.length <= 104
	trust[i].length == 2
	All the pairs of trust are unique.
	ai != bi
	1 <= ai, bi <= n

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
		// let n = n as usize;
		// let mut trust_in = vec![vec![]; n + 1];
		// let mut trust_by = trust_in.clone();
		// for v in trust {
		// 	let x = v[0] as usize;
		// 	let y = v[1] as usize;
		// 	trust_in[x].push(y);
		// 	trust_by[y].push(x);
		// }
		// trust_by[1..].iter()
		// 	.enumerate()
		// 	.filter(|(i, v)| v.len() == n - 1 && trust_in[*i + 1].is_empty())
		// 	.map(|(i, _)| i as i32 + 1)
		// 	.next().unwrap_or(-1)
		let n = n as usize;
		let mut trust_metric = vec![0; n + 1];
		for v in trust {
			let x = v[0] as usize;
			let y = v[1] as usize;
			trust_metric[x] -= 1; // if x trust y, x minus 1 point
			trust_metric[y] += 1; // if x trust y, y plus 1 point
		}
		trust_metric[1..].iter().position(|&x| x == n - 1).map_or(-1, |i| i as i32 + 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		assert_eq!(1, Solution::find_judge(1, vec![]));
    }
}
