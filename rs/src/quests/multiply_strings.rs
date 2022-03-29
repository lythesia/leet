/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 
Example 1:
Input: num1 = "2", num2 = "3"
Output: "6"
Example 2:
Input: num1 = "123", num2 = "456"
Output: "56088"
 
Constraints:

	1 <= num1.length, num2.length <= 200
	num1 and num2 consist of digits only.
	Both num1 and num2 do not contain any leading zero, except the number 0 itself.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut ans = vec![0; num1.len() + num2.len()];
        let mut c = 0;
        for (mut lsd, &i) in num2.as_bytes().iter().rev().enumerate() {
            for &j in num1.as_bytes().iter().rev() {
                let p = (i - b'0')*(j - b'0') + c;
                ans[lsd] += p;
                c = ans[lsd]/10;
                ans[lsd] %= 10;
                lsd += 1;
            }
            ans[lsd] += c;
            c = 0;
        }
        // trim right zeroes
        ans.reverse();
        if let Some(i) = ans.iter().position(|&x|x != 0) {
            ans[i..].iter().map(|&x| (x + b'0') as char).collect()
        } else { "0".into() }
    }
}

// submission codes end

use core::num;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("56088".to_owned(), Solution::multiply("123".into(), "456".into()));
    }
}
