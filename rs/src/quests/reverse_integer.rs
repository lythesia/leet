/**
 * [7] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.

Example 1:

Input: 123
Output: 321


Example 2:

Input: -123
Output: -321


Example 3:

Input: 120
Output: 21


Note:
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut n, neg) = ((x as i64).abs(), x < 0);
        let mut ans = 0i64;
        while n != 0 {
            ans = ans*10 + n%10;
            n /= 10;
        }
        if neg {
            if -ans < std::i32::MIN.into() { 0 } else { -ans as i32 }
        } else {
            if ans > std::i32::MAX.into() { 0 } else { ans as i32 }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
