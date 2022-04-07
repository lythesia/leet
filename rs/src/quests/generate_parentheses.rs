/**
 * [22] Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 
Example 1:
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
Example 2:
Input: n = 1
Output: ["()"]
 
Constraints:

	1 <= n <= 8

 */
pub struct Solution {}

// submission codes start here

fn dfs(l: i32, r: i32, s: &mut String, v: &mut Vec<String>) {
    if l == 0 && r == 0 {
        v.push(s.clone());
    } else {
        // use ( first
        if l > 0 {
            s.push('(');
            dfs(l - 1, r, s, v);
            s.pop();
        }
        // if can close
        if l < r {
            s.push(')');
            dfs(l, r - 1, s, v);
            s.pop();
        }
    }
}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut s = String::new();
        let mut v = vec![];
        dfs(n, n, &mut s, &mut v);
        v
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
