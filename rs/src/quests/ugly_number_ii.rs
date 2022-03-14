/**
 * [264] Ugly Number II
 *
 * An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
Given an integer n, return the nth ugly number.
 
Example 1:
Input: n = 10
Output: 12
Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.

Example 2:
Input: n = 1
Output: 1
Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.

 
Constraints:

	1 <= n <= 1690

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // ugly[n] must be min(ugly[i]*5, ugly[j]*3, ugly[k]*2) which all > ugly[n-1], with i < j < k
    // (即第n个一定是由前面n-1个中的某个数计算而来, 要么x*2 或 y*3 或z*5, 且一定是z(ugly[i]) < y(ugly[j]) < x(ugly[k]))
    // 假设本次第n-1的min为ugly[k]*2, 那么下次依旧在前n个数中选, 那么*2的候选数一定是ugly[k + 1], 如果还是ugly[k], 那么已经算过了没意义, 而它的next_greater只能是ugly[k+1]
    // *3和*5同理
    // 注意如果同时有多个候选数分别*2 *3 *5得到相同的min, 则他们都要各自往后一个next_greater
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        let (mut candidate_to_mul_2, mut candidate_to_mul_3, mut candidate_to_mul_5) = (1, 1, 1);
        for i in 2..=n {
            dp[i] = *[dp[candidate_to_mul_2]*2, dp[candidate_to_mul_3]*3, dp[candidate_to_mul_5]*5].iter().min().unwrap();
            if dp[i] == dp[candidate_to_mul_2]*2 { candidate_to_mul_2 += 1; }
            if dp[i] == dp[candidate_to_mul_3]*3 { candidate_to_mul_3 += 1; }
            if dp[i] == dp[candidate_to_mul_5]*5 { candidate_to_mul_5 += 1; }
        }
        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}
