/**
 * [740] Delete and Earn
 *
 * You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:

	Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.

Return the maximum number of points you can earn by applying the above operation some number of times.
 
Example 1:
Input: nums = [3,4,2]
Output: 6
Explanation: You can perform the following operations:
- Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
- Delete 2 to earn 2 points. nums = [].
You earn a total of 6 points.

Example 2:
Input: nums = [2,2,3,3,3,4]
Output: 9
Explanation: You can perform the following operations:
- Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
- Delete a 3 again to earn 3 points. nums = [3].
- Delete a 3 once more to earn 3 points. nums = [].
You earn a total of 9 points.
 
Constraints:

	1 <= nums.length <= 2 * 104
	1 <= nums[i] <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        use std::cmp::max;
        let mut map = BTreeMap::new();
        for x in nums {
            *map.entry(x).or_insert(0) += 1;
        }
        let n = map.len();
        let mut it = map.into_iter();
        let fst = it.next().unwrap();
        let mut last = fst.0;
        let (mut dp_lea_last, mut dp_del_last) = (0, fst.0*fst.1);
        while let Some((v, c)) = it.next() {
            let keep = dp_lea_last;
            dp_lea_last = max(dp_lea_last, dp_del_last); // not delete
            // delete
            dp_del_last = if v-1 != last && dp_del_last > keep {
                dp_del_last + v*c
            } else {
                keep + v*c
            };
            last = v;
        }
        max(dp_lea_last, dp_del_last)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(6, Solution::delete_and_earn(vec![3,4,2]));
        // assert_eq!(9, Solution::delete_and_earn(vec![2,2,3,3,3,4]));
        assert_eq!(43, Solution::delete_and_earn(vec![1,6,3,3,8,4,8,10,1,3]));
    }
}
