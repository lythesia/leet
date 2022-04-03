/**
 * [435] Non-overlapping Intervals
 *
 * Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
 
Example 1:
Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
Output: 1
Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.

Example 2:
Input: intervals = [[1,2],[1,2],[1,2]]
Output: 2
Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.

Example 3:
Input: intervals = [[1,2],[2,3]]
Output: 0
Explanation: You don't need to remove any of the intervals since they're already non-overlapping.

 
Constraints:

	1 <= intervals.length <= 105
	intervals[i].length == 2
	-5 * 104 <= starti < endi <= 5 * 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // sort by right
        intervals.sort_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
        intervals.into_iter()
            .fold((i32::MIN, 0), |(last_end, ans), i| {
                if i[0] < last_end {
                    (last_end, ans + 1) // keep last_end smaller if overlap
                } else {
                    (i[1], ans) // update last_end with new interval if not overlap
                }
            }).1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::erase_overlap_intervals(vec_vec![[1,2],[1,2],[1,2]]));
        assert_eq!(1, Solution::erase_overlap_intervals(vec_vec![[1,2],[2,3],[3,4],[1,3]]));
        assert_eq!(2, Solution::erase_overlap_intervals(vec_vec![[0,2],[1,3],[2,4],[3,5],[4,6]]));
    }
}
