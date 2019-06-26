/**
 * [6] ZigZag Conversion
 *
 * cyclehe string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R


And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);

Example 1:

Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"


Example 2:

Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:

P     I    N
A   L S  I G
Y A   H R
P     I

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn convert(s: String, n: i32) -> String {
        let n = n as usize;
        if s.is_empty() || n == 1 { return s; }
        let mut ans = String::with_capacity(s.len());
        let ss: Vec<char> = s.chars().collect();
        let len = s.len();
        let cycle = 2*n - 2;
        for i in 0..n {
            let mut j = 0;
            while i + j < len {
                ans.push(ss[i + j]);
                if i != 0 && i != n - 1 && j + cycle - i < len {
                    ans.push(ss[j + cycle - i]);
                }
                j += cycle;
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("PAHNAPLSIIGYIR".to_owned(), Solution::convert("PAYPALISHIRING".to_owned(), 3));
        assert_eq!("PINALSIGYAHRPI".to_owned(), Solution::convert("PAYPALISHIRING".to_owned(), 4));
    }
}
