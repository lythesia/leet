/**
 * [977] Squares of a Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
 
Example 1:
Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].

Example 2:
Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]

 
Constraints:

	1 <= nums.length <= 104
	-104 <= nums[i] <= 104
	nums is sorted in non-decreasing order.

 
Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // simple ver: 8ms
        let n = nums.len();
        let mut ans = vec![0; n];
        let (mut l, mut r) = (0, n - 1);
        let mut cnt = n;
        while cnt != 0 {
            if nums[l].abs() < nums[r].abs() {
                ans[cnt - 1] = nums[r].pow(2);
                r -= 1;
            } else {
                ans[cnt - 1] = nums[l].pow(2);
                l += 1;
            }
            cnt -= 1;
        }
        ans

        /*: 0ms
        let n = nums.len();
        let mut ans = Vec::with_capacity(n);
        let sq = |x: i32| x*x;
        if nums[0] ^ nums[n-1] < 0 {
            let i = nums.binary_search(&0).unwrap_or_else(|i| i); // >= 1
            let (mut l, mut r) = ((0..i).rev().peekable(), (i..n).peekable());
            loop {
                match (l.peek(), r.peek()) {
                    (Some(ll), Some(rr)) => {
                        let (x, y) = (sq(nums[*ll]), sq(nums[*rr]));
                        if x < y {
                            ans.push(x);
                            l.next();
                        } else {
                            ans.push(y);
                            r.next();
                        }
                    },
                    (Some(ll), _) => {
                        ans.push(sq(nums[*ll]));
                        l.next();
                    },
                    (_, Some(rr)) => {
                        ans.push(sq(nums[*rr]));
                        r.next();
                    },
                    _ => break,
                }
            }
            ans
        } else {
            // one elem falls here
            if nums[0] >= 0 {
                (0..n).map(|x| sq(nums[x])).collect()
            } else {
                (0..n).rev().map(|x| sq(nums[x])).collect()
            }
        }
        */
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0,1,9,16,100], Solution::sorted_squares(vec![-4,-1,0,3,10]));
    }
}
