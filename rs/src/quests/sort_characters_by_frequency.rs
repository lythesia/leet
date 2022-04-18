/**
 * [451] Sort Characters By Frequency
 *
 * Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.
Return the sorted string. If there are multiple answers, return any of them.
 
Example 1:
Input: s = "tree"
Output: "eert"
Explanation: 'e' appears twice while 'r' and 't' both appear once.
So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.

Example 2:
Input: s = "cccaaa"
Output: "aaaccc"
Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
Note that "cacaca" is incorrect, as the same characters must be together.

Example 3:
Input: s = "Aabb"
Output: "bbAa"
Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
Note that 'A' and 'a' are treated as two different characters.

 
Constraints:

	1 <= s.length <= 5 * 105
	s consists of uppercase and lowercase English letters and digits.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        use std::collections::BinaryHeap;
        let mut freq = [0; 128];
        for c in s.chars() {
            freq[c as usize] += 1;
        }
        let mut heap = BinaryHeap::new();
        for (i, c) in freq.iter().enumerate().filter(|(_, c)| **c > 0) {
            heap.push((*c, unsafe {char::from_u32_unchecked(i as u32)}));
        }
        let mut ans = String::new();
        while let Some((top, c)) = heap.pop() {
            ans.extend(std::iter::repeat(c).take(top));
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
        println!("{}", Solution::frequency_sort("tree".into()));
        println!("{}", Solution::frequency_sort("cccaaa".into()));
        println!("{}", Solution::frequency_sort("Aabb".into()));
    }
}
