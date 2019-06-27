/**
 * [8] String to Integer (atoi)
 *
 * Implement atoi which converts a string to an integer.

The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.

The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.

If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.

If no valid conversion could be performed, a zero value is returned.

Note:


    Only the space character ' ' is considered as whitespace character.
    Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.


Example 1:

Input: "42"
Output: 42


Example 2:

Input: "   -42"
Output: -42
Explanation: The first non-whitespace character is '-', which is the minus sign.
             Then take as many numerical digits as possible, which gets 42.


Example 3:

Input: "4193 with words"
Output: 4193
Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.


Example 4:

Input: "words and 987"
Output: 0
Explanation: The first non-whitespace character is 'w', which is not a numerical
             digit or a +/- sign. Therefore no valid conversion could be performed.

Example 5:

Input: "-91283472332"
Output: -2147483648
Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
             Thefore INT_MIN (−231) is returned.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut ans = 0i64;
        let mut sign = false;
        let mut neg = false;
        let mut num = false;
        let ss = s.trim_start().trim_end_matches(|c: char| !c.is_numeric());
        for c in ss.chars() {
            if c == '+' {
                if !sign { sign = true; neg = false; continue; }
                else {
                    if num { break; } else { return 0; }
                }
            }
            if c == '-' {
                if !sign { sign = true; neg = true; continue; }
                else {
                    if num { break; } else { return 0; }
                }
            }
            if c.is_numeric() {
                sign = true;
                num = true;
                ans = ans*10 + (c.to_digit(10).unwrap() as i64);
                if !neg && ans > std::i32::MAX.into() { return std::i32::MAX; }
                if neg && -ans < std::i32::MIN.into() { return std::i32::MIN; }
            } else {
                if num { break; } else { return 0; }
            }
        }
        if !neg {
            ans as i32
        } else {
            -ans as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42);
        assert_eq!(Solution::my_atoi(" -42".to_owned()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_owned()), 1);
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648);
        assert_eq!(Solution::my_atoi("3.14".to_owned()), 3);
        assert_eq!(Solution::my_atoi("9223372036854775808".to_owned()), 2147483647);
        assert_eq!(Solution::my_atoi("-13+8".to_owned()), 13);
    }
}
