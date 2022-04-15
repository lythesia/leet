/**
 * [201] Bitwise AND of Numbers Range
 *
 * Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.
 
Example 1:
Input: left = 5, right = 7
Output: 4

Example 2:
Input: left = 0, right = 0
Output: 0

Example 3:
Input: left = 1, right = 2147483647
Output: 0

 
Constraints:

	0 <= left <= right <= 231 - 1

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let (mut left, right) = (left as i64, right as i64);
        let low_bit = |x: i64| x & (-x);
        loop {
            let low = low_bit(left);
            let next = (left & (!low)) + (low << 1);
            if next == 0 || next > right { break; }
            if next < right {
                left &= !low;
            } else if next == right {
                left &= right;
                break;
            }
        }
        left as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
        assert_eq!(0, Solution::range_bitwise_and(3, 5));
        assert_eq!(0, Solution::range_bitwise_and(1, 0x7fffffff));
    }
}
