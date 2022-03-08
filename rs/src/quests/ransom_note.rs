/**
 * [383] Ransom Note
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be constructed from magazine and false otherwise.
Each letter in magazine can only be used once in ransomNote.
 
Example 1:
Input: ransomNote = "a", magazine = "b"
Output: false
Example 2:
Input: ransomNote = "aa", magazine = "ab"
Output: false
Example 3:
Input: ransomNote = "aa", magazine = "aab"
Output: true
 
Constraints:

	1 <= ransomNote.length, magazine.length <= 105
	ransomNote and magazine consist of lowercase English letters.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut h = [0; 26];
        magazine.as_bytes().iter().for_each(|&b| h[(b - b'a') as usize] += 1);
        for &b in ransom_note.as_bytes() {
            let c = &mut h[(b - b'a') as usize];
            *c -= 1;
            if *c < 0 {
                return false;
            }
        }
        true
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
