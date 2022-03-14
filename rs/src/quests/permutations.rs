/**
 * [46] Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 
Example 1:
Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
Example 2:
Input: nums = [0,1]
Output: [[0,1],[1,0]]
Example 3:
Input: nums = [1]
Output: [[1]]
 
Constraints:

	1 <= nums.length <= 6
	-10 <= nums[i] <= 10
	All the integers of nums are unique.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn dfs(cnt: usize, nums: &Vec<i32>, v: &mut Vec<i32>, vis: &mut Vec<bool>, ans: &mut Vec<Vec<i32>>) {
        if cnt == nums.len() {
            ans.push(v.clone());
            return;
        }
        for (i, &x) in nums.iter().enumerate() {
            if !vis[i] {
                // take i
                vis[i] = true;
                v.push(x);
                Self::dfs(cnt + 1, nums, v, vis, ans);
                // pop i
                vis[i] = false;
                v.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut vis = vec![false; nums.len()];
        let mut v = vec![];
        let mut ans = vec![];
        Self::dfs(0, &nums, &mut v, &mut vis, &mut ans);
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::permute(vec![1,2,3]));
        println!("{:?}", Solution::permute(vec![0,1]));
        println!("{:?}", Solution::permute(vec![1]));
    }
}
