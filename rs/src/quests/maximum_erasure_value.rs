/**
 * [1695] Maximum Erasure Value
 *
 * You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.
Return the maximum score you can get by erasing exactly one subarray.
An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).
 
Example 1:
Input: nums = [4,2,4,5,6]
Output: 17
Explanation: The optimal subarray here is [2,4,5,6].

Example 2:
Input: nums = [5,2,1,2,5,2,1,2,5]
Output: 8
Explanation: The optimal subarray here is [5,2,1] or [1,2,5].

 
Constraints:

	1 <= nums.length <= 105
	1 <= nums[i] <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut ret, mut mmax) = (0, 0);
        let mut set = std::collections::HashSet::new();
        let mut l = 0;
        for nr in &nums {
            if !set.contains(nr) {
                set.insert(*nr);
                mmax += nr;
            } else {
                // shrink left
                loop {
                    let nl = &nums[l];
                    if nl != nr {
                        set.remove(nl);
                        mmax -= nl;
                        l += 1;
                    } else {
                        l += 1;
                        break;
                    }
                }
            }
            ret = std::cmp::max(ret, mmax);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(17, Solution::maximum_unique_subarray(vec![4,2,4,5,6]));
        assert_eq!(8, Solution::maximum_unique_subarray(vec![5,2,1,2,5,2,1,2,5]));
    }
}
