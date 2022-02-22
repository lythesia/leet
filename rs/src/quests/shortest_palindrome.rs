/**
 * [214] Shortest Palindrome
 *
 * You are given a string s. You can convert s to a palindrome by adding characters in front of it.
Return the shortest palindrome you can find by performing this transformation.
 
Example 1:
Input: s = "aacecaaa"
Output: "aaacecaaa"
Example 2:
Input: s = "abcd"
Output: "dcbabcd"
 
Constraints:

	0 <= s.length <= 5 * 104
	s consists of lowercase English letters only.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    fn manachar(s: &String) -> usize {
        let n = s.len();
        let mut a = String::with_capacity(n + n+1 +2);
        a.push('^');
        s.chars().for_each(|c| {a.push('#'); a.push(c);});
        a.push_str("#$");
        let nn = 2*n+1;
        let mut p = vec![0; nn];
        let (mut id, mut mx) = (0usize, 0usize);
        let (mut mmax, mut e) = (0, 0);

        let b = a.as_bytes();
        for i in 1..nn { // from '#'
            let mut pi = if i < mx {
                let j = 2*id - i; // mx' j id i mx: id - j = i - id
                std::cmp::min(mx - i + 1, p[j])
            } else {1};
            // extend
            while b[i - pi] == b[i + pi] { pi += 1; }
            // update
            p[i] = pi;
            if i + pi - 1 > mx {
                mx = i + pi - 1;
                id = i;
            }
            if pi > mmax {
                mmax = pi;
                let ss = 2*id - mx;
                let ee = mx;
                if (ss - 1)/2 == 0 { e = ee; } // 0=^, 1=#, 2=real-char
            }
        }
        if e & 1 == 1 { (e - 1)/2 - 1 }
        else { (e - 1)/2 }
    }

    // find largest pali starts from index 0!
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() { s }
        else {
            let d = s.len() - Self::manachar(&s) - 1;
            s.chars().rev().take(d).collect::<String>() + &s
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("aaacecaaa".to_owned(), Solution::shortest_palindrome("aacecaaa".into()));
        assert_eq!("aaacecbabcecaaa".to_owned(), Solution::shortest_palindrome("abcecaaa".into()));
        assert_eq!("dcbabcd".to_owned(), Solution::shortest_palindrome("abcd".into()));
        assert_eq!("aaaadcaaaacdaaaa".to_owned(), Solution::shortest_palindrome("aaaacdaaaa".into()));
    }
}
