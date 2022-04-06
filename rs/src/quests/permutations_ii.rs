/**
 * [47] Permutations II
 *
 * Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
 
Example 1:
Input: nums = [1,1,2]
Output:
[[1,1,2],
 [1,2,1],
 [2,1,1]]

Example 2:
Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

 
Constraints:

	1 <= nums.length <= 8
	-10 <= nums[i] <= 10

 */
pub struct Solution {}

// submission codes start here

fn next_permutation(a: &mut Vec<i32>) -> bool {
    let mut i = a.len() - 1;
    while i > 0 && a[i-1] >= a[i] {
        i -= 1;
    }
    if i > 0 {
        let mut j = a.len() - 1;
        while j > i-1 && a[j] <= a[i-1] {
            j -= 1;
        }
        a.swap(i-1, j);
        a[i..].reverse();
        true
    } else {
        false
    }
}
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        nums.sort();
        loop {
            ans.push(nums.clone());
            if !next_permutation(&mut nums) { break; }
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
        println!("{:?}", Solution::permute_unique(vec![1,1,2]));
        println!("{:?}", Solution::permute_unique(vec![1,2,3]));
    }
}
