/**
 * [1014] Best Sightseeing Pair
 *
 * You are given an integer array values where values[i] represents the value of the ith sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.
The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.
Return the maximum score of a pair of sightseeing spots.
 
Example 1:
Input: values = [8,1,5,2,6]
Output: 11
Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11

Example 2:
Input: values = [1,2]
Output: 2

 
Constraints:

	2 <= values.length <= 5 * 104
	1 <= values[i] <= 1000

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // [8,1,5,2,6]
    // contrib to left(0)
    //  0 1 2 3  4 5

    // 0: 0 3 -1 2
    // 1:   4 0  3
    // 2:     1  4
    // 3:        5
    // ?:        
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = values.len();
        let mut max_for_next_i = n - 1;
        let mut ans = 0;
        for i in (0..n-1).rev() {
            // max_for_i for pair(i, max_for_i) with max_for_i > i
            ans = max(ans, values[i] + values[max_for_next_i] - (max_for_next_i - i) as i32);
            // re calc max_for_next_i
            let curr_contrib = values[i] - 1;
            let last_max_contrib = values[max_for_next_i] + (i as i32 - 1 - max_for_next_i as i32);
            if curr_contrib > last_max_contrib {
                max_for_next_i = i;
            }
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
        assert_eq!(11, Solution::max_score_sightseeing_pair(vec![8,1,5,2,6]));
        assert_eq!(2, Solution::max_score_sightseeing_pair(vec![1,2]));
    }
}
