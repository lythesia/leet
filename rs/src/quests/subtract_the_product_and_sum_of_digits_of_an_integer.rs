/**
 * [1281] Subtract the Product and Sum of Digits of an Integer
 *
 * Given an integer number n, return the difference between the product of its digits and the sum of its digits.
 
Example 1:
Input: n = 234
Output: 15 
Explanation: 
Product of digits = 2 * 3 * 4 = 24 
Sum of digits = 2 + 3 + 4 = 9 
Result = 24 - 9 = 15

Example 2:
Input: n = 4421
Output: 21
Explanation: 
Product of digits = 4 * 4 * 2 * 1 = 32 
Sum of digits = 4 + 4 + 2 + 1 = 11 
Result = 32 - 11 = 21

 
Constraints:

	1 <= n <= 10^5

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (p, s) = std::iter::successors(Some(n), |&x| Some(x/10))
            .take_while(|&x| x > 0)
            .map(|x| x%10)
            .fold((1, 0), |(p, s), x| (p*x, s+x));
        p - s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
        assert_eq!(21, Solution::subtract_product_and_sum(4421));
    }
}
