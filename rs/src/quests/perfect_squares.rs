/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 
Example 1:
Input: n = 12
Output: 3
Explanation: 12 = 4 + 4 + 4.

Example 2:
Input: n = 13
Output: 2
Explanation: 13 = 4 + 9.

 
Constraints:

	1 <= n <= 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // Legendre's three-square theorem
    // 1. 任何正整数都能表示成四个平方数的和
    // 2. 如果正整数 n 被表示为三个平方数的和，那么 n 不等于 4^a * (8b + 7) (同时也不能表示为1个或2个的平方和, 很简单, 补0^2能构成三个)
    pub fn num_squares(mut n: i32) -> i32 {
        let is_square = |n: i32| {
            let x = f32::sqrt(n as f32) as i32;
            x*x == n
        };
        if is_square(n) { 1 }
        else {
            while n % 4 == 0 {
                n /= 4;
            }
            if n % 8 == 7 {
                4
            } else {
                let mut i = 1;
                while i*i < n {
                    if is_square(n - i*i) {
                        return 2;
                    }
                    i += 1;
                }
                3
            }
        }
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
