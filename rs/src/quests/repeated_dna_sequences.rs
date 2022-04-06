/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.

	For example, "ACGAATTCCG" is a DNA sequence.

When studying DNA, it is useful to identify repeated sequences within the DNA.
Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 
Example 1:
Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
Output: ["AAAAACCCCC","CCCCCAAAAA"]
Example 2:
Input: s = "AAAAAAAAAAAAA"
Output: ["AAAAAAAAAA"]
 
Constraints:

	1 <= s.length <= 105
	s[i] is either 'A', 'C', 'G', or 'T'.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // little 3bit
    // A 65 ..001
    // C 67 ..011
    // G 71 ..111
    // T 84 ..100
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ans = vec![];
        let mut h = std::collections::HashMap::new();
        let mut t = 0;
        for (i, &c) in s.as_bytes().iter().enumerate() {
            t = (t<<3) & 0x3fffffff | (c as i32 & 0x7); // every 3bit represents a letter, 3*10 => a 10-letter-dna has 30bits, thus 0x3fff,ffff
            let e = h.entry(t).or_insert(0);
            if *e == 1 {
                ans.push(s[i-9..i+1].to_string());
            }
            *e += 1;
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
    }
}
