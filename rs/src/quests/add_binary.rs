/**
 * [67] Add Binary
 *
 * Given two binary strings a and b, return their sum as a binary string.
 
Example 1:
Input: a = "11", b = "1"
Output: "100"
Example 2:
Input: a = "1010", b = "1011"
Output: "10101"
 
Constraints:

	1 <= a.length, b.length <= 104
	a and b consist only of '0' or '1' characters.
	Each string does not contain leading zeros except for the zero itself.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ans = String::new();
        let (mut it_a, mut it_b) = (a.as_bytes().iter().rev(), b.as_bytes().iter().rev());
        let mut c = 0;
        loop {
            match (it_a.next(), it_b.next()) {
                (Some(&i), Some(&j)) => {
                    let s = (i - b'0') + (j - b'0') + c;
                    ans.insert(0, (s%2 + b'0') as char);
                    c = s/2;
                },
                (_, Some(&x)) | (Some(&x), _) => {
                    let s = x - b'0' + c;
                    ans.insert(0, (s%2 + b'0') as char);
                    c = s/2;
                }
                _ => break,
            }
        }
        if c > 0 { ans.insert(0, (c + b'0') as char)}
        ans
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
