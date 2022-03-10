/**
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 *
 * You are given an array prices where prices[i] is the price of a given stock on the ith day, and an integer fee representing a transaction fee.
Find the maximum profit you can achieve. You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.
Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 
Example 1:
Input: prices = [1,3,2,8,4,9], fee = 2
Output: 8
Explanation: The maximum profit can be achieved by:
- Buying at prices[0] = 1
- Selling at prices[3] = 8
- Buying at prices[4] = 4
- Selling at prices[5] = 9
The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.

Example 2:
Input: prices = [1,3,7,5,10,3], fee = 3
Output: 6

 
Constraints:

	1 <= prices.length <= 5 * 104
	1 <= prices[i] < 5 * 104
	0 <= fee < 5 * 104

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp_own[i] = max: dp_not_own[i-1] - a[i], dp_own[i-1]
    // dp_not_own[i] = max: dp_own[i-1] + a[i] - fee, dp_not_own[i-1]
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        if n == 1 { return 0; }
        else {
            use std::cmp::max;
            let (mut dp_own, mut dp_not) = ([0; 2], [0; 2]);
            dp_own[0] = -prices[0];
            dp_not[0] = 0;
            let mut idx = 0;
            for &v in &prices[1..] {
                // flip idx
                idx = 1 - idx;
                dp_own[idx] = max(dp_not[1-idx] - v, dp_own[1-idx]);
                dp_not[idx] = max(dp_own[1-idx] + v - fee, dp_not[1-idx]);
            }
            max(dp_own[idx], dp_not[idx])
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(8, Solution::max_profit(vec![1,3,2,8,4,9], 2));
        assert_eq!(6, Solution::max_profit(vec![1,3,7,5,10,3], 3));
    }
}
