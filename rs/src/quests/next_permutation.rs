/**
 * [31] Next Permutation
 *
 * A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

	For example, for arr = [1,2,3], the following are considered permutations of arr: [1,2,3], [1,3,2], [3,1,2], [2,3,1].

The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the next permutation of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).

	For example, the next permutation of arr = [1,2,3] is [1,3,2].
	Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
	While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.

Given an array of integers nums, find the next permutation of nums.
The replacement must be in place and use only constant extra memory.
 
Example 1:
Input: nums = [1,2,3]
Output: [1,3,2]

Example 2:
Input: nums = [3,2,1]
Output: [1,2,3]

Example 3:
Input: nums = [1,1,5]
Output: [1,5,1]

 
Constraints:

	1 <= nums.length <= 100
	0 <= nums[i] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let nums = nums.as_mut_slice(); // speed up
        let mut i = nums.len() - 1;
        // from [tail to 0], find first pair a[i-1] < a[i]
        // it means after a[i], all permutations exhaust (a reversed or descending seq is the last perm of the seq)
        while i > 0 {
            if nums[i-1] < nums[i] {break;}
            i -= 1;
        }
        // if found, [0, i] is asc, [i, end] is desc
        if i > 0 {
            let mut j = nums.len() - 1;
            // 1 2 3 7 6 5 4, next = 1 2 4 3 5 6 7, a[i] = 7, so next a[i-1] should be succesive of itself
            // from [tail to i], find first elem a[j] > a[i-1]
            // it means find first succesive of a[i-1]
            while j > i - 1 {
                if nums[j] > nums[i-1] {break;}
                j -= 1;
            }
            // and then swap them
            nums.swap(j, i - 1);
            // at last sort rest, or just reverse, since rest is desc
            // nums[i..].sort();
        }
        // else {
        // if not found, the whole seq is the last perm of itself, reverse it, and get loop at all start perm
        //     nums.reverse();
        // }
        nums[i..].reverse();
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
