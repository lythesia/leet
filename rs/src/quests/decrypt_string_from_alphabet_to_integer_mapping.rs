/**
 * [1309] Decrypt String from Alphabet to Integer Mapping
 *
 * You are given a string s formed by digits and '#'. We want to map s to English lowercase characters as follows:

	Characters ('a' to 'i') are represented by ('1' to '9') respectively.
	Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.

Return the string formed after mapping.
The test cases are generated so that a unique mapping will always exist.
 
Example 1:
Input: s = "10#11#12"
Output: "jkab"
Explanation: "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".

Example 2:
Input: s = "1326#"
Output: "acz"

 
Constraints:

	1 <= s.length <= 1000
	s consists of digits and the '#' letter.
	s will be a valid string such that mapping is always possible.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut ans = String::new();
        let mut sharp = false;
        let mut lastd = 10u8;
        for c in s.chars().rev() {
            if c == '#' {
                sharp = true;
            } else if let Some(d) = c.to_digit(10) {
                if sharp {
                    if lastd != 10 {
                        ans.push(((d as u8)*10 + lastd - 1 + b'a') as char);
                        lastd = 10;
                        sharp = false;
                    } else {
                        lastd = d as u8;
                    }
                } else {
                    ans.push((d as u8 - 1 + b'a') as char);
                }
            }
        }
        ans.chars().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("jkab".to_owned(), Solution::freq_alphabets("10#11#12".into()));
        assert_eq!("acz".to_owned(), Solution::freq_alphabets("1326#".into()));
    }
}
