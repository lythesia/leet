/**
 * [343] Integer Break
 *
 * Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
Return the maximum product you can get.
 
Example 1:
Input: n = 2
Output: 1
Explanation: 2 = 1 + 1, 1 × 1 = 1.

Example 2:
Input: n = 10
Output: 36
Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.

 
Constraints:

	2 <= n <= 58

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // math: all factor into 3,3,3,...
    pub fn integer_break(mut n: i32) -> i32 {
        match n {
            0|1 => unreachable!(),
            2 => 1,
            3 => 2,
            4 => 4,
            _ => {
                let mut ans = 1;
                while n > 4 {
                    ans *= 3;
                    n -= 3;
                }
                ans *= n;
                ans
            },
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::integer_break(3));
        assert_eq!(36, Solution::integer_break(10));
    }
}
