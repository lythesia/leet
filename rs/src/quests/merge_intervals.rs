/**
 * [56] Merge Intervals
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
 
Example 1:
Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].

Example 2:
Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.

 
Constraints:

	1 <= intervals.length <= 104
	intervals[i].length == 2
	0 <= starti <= endi <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|v| v[0]);
        let mut i = 0;
        use std::cmp::max;
        while i + 1 < intervals.len() {
            let (x, y) = (&intervals[i], &intervals[i + 1]);
            if x[1] >= y[0] {
                let z = vec![x[0], max(x[1], y[1])];
                intervals.remove(i);
                intervals.remove(i);
                intervals.insert(i, z);
            } else {
                i += 1;
            }
        }
        intervals
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Solution::merge(vec_vec![[1,3],[2,6],[8,10],[15,18]]);
    }
}
