/**
 * [202] Happy Number
 *
 * Write an algorithm to determine if a number n is happy.
A happy number is a number defined by the following process:

	Starting with any positive integer, replace the number by the sum of the squares of its digits.
	Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
	Those numbers for which this process ends in 1 are happy.

Return true if n is a happy number, and false if not.
 
Example 1:
Input: n = 19
Output: true
Explanation:
12 + 92 = 82
82 + 22 = 68
62 + 82 = 100
12 + 02 + 02 = 1

Example 2:
Input: n = 2
Output: false

 
Constraints:

	1 <= n <= 231 - 1

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let f = |n: i32| -> i32 {
            std::iter::successors(Some(n), |&x| Some(x/10))
            .take_while(|&x| x > 0)
            .map(|x| (x%10).pow(2))
            .sum()
        };
        let mut set = std::collections::HashSet::new();
        loop {
            n = f(n);
            if n == 1 { return true; }
            if set.contains(&n) { return false; }
            set.insert(n);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(!Solution::is_happy(2));
    }
}
