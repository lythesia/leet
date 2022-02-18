/**
 * [415] Add Strings
 *
 * Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
 
Example 1:
Input: num1 = "11", num2 = "123"
Output: "134"

Example 2:
Input: num1 = "456", num2 = "77"
Output: "533"

Example 3:
Input: num1 = "0", num2 = "0"
Output: "0"

 
Constraints:

	1 <= num1.length, num2.length <= 104
	num1 and num2 consist of only digits.
	num1 and num2 don't have any leading zeros except for the zero itself.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, num2) = if num1.len() < num2.len() {
            (num2, num1)
        } else {
            (num1, num2)
        };

        let mut v = vec![];
        let mut c = 0_u8;
        let len = num1.len() - num2.len();
        for (x, y) in num1.bytes().rev().zip(num2.bytes().rev()) {
            let s = x - b'0' + y - b'0' + c;
            v.push((s % 10 + b'0') as char);
            c = s / 10;
        }
        for x in num1[..len].bytes().rev() {
            let s = x - b'0' + c;
            v.push((s % 10 + b'0') as char);
            c = s / 10;
        }
        if c > 0 { v.push((c + b'0') as char); }

        v.iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("134"), Solution::add_strings(String::from("11"), String::from("123")));
        assert_eq!(String::from("533"), Solution::add_strings(String::from("456"), String::from("77")));
        assert_eq!(String::from("0"), Solution::add_strings(String::from("0"), String::from("0")));
    }
}
