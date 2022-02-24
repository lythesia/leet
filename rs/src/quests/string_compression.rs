/**
 * [443] String Compression
 *
 * Given an array of characters chars, compress it using the following algorithm:
Begin with an empty string s. For each group of consecutive repeating characters in chars:

	If the group's length is 1, append the character to s.
	Otherwise, append the character followed by the group's length.

The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.
After you are done modifying the input array, return the new length of the array.
You must write an algorithm that uses only constant extra space.
 
Example 1:
Input: chars = ["a","a","b","b","c","c","c"]
Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".

Example 2:
Input: chars = ["a"]
Output: Return 1, and the first character of the input array should be: ["a"]
Explanation: The only group is "a", which remains uncompressed since it's a single character.

Example 3:
Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".
 
Constraints:

	1 <= chars.length <= 2000
	chars[i] is a lowercase English letter, uppercase English letter, digit, or symbol.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut ch, mut s) = (0 as char, 0);
        let n = chars.len();
        for i in 0..n {
            let c = chars[i];
            // new char
            if c != ch {
                if s + 1 < i {
                    let ns = (i - s).to_string();
                    let mut ss = s + 1;
                    for cc in ns.chars() {
                        chars[ss] = cc;
                        ss += 1;
                    }
                    while ss < i {
                        chars[ss] = 0 as char;
                        ss += 1;
                    }
                }
                ch = c;
                s = i;
            }
        }
        // last
        if s < n - 1 {
            let ns = (n - s).to_string();
            let mut ss = s + 1;
            for cc in ns.chars() {
                chars[ss] = cc;
                ss += 1;
            }
            while ss < n {
                chars[ss] = 0 as char;
                ss += 1;
            }
        }
        chars.retain(|c| *c != 0 as char);
        chars.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v1 = vec!['a','a','b','b','c','c','c'];
        assert_eq!(6, Solution::compress(&mut v1));
        assert_eq!(vec!['a','2','b','2','c','3'], v1);

        let mut v2= vec!['a'];
        assert_eq!(1, Solution::compress(&mut v2));
        assert_eq!(vec!['a'], v2);

        let mut v3 = vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        assert_eq!(4, Solution::compress(&mut v3));
        assert_eq!(vec!['a','b','1','2'], v3);
    }
}
