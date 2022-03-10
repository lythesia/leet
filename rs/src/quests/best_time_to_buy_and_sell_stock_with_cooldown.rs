/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * You are given an array prices where prices[i] is the price of a given stock on the ith day.
Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:

	After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).

Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 
Example 1:
Input: prices = [1,2,3,0,2]
Output: 3
Explanation: transactions = [buy, sell, cooldown, buy, sell]

Example 2:
Input: prices = [1]
Output: 0

 
Constraints:

	1 <= prices.length <= 5000
	0 <= prices[i] <= 1000

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp_buy[i] = max: dp_sell[i-2] - a[i], dp_buy[i-1]
    // dp_sell[i] = max: dp_buy[i-1] + a[i], dp_sell[i-1]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 1 { return 0; }
        use std::cmp::max;
        let (mut dp_buy, mut dp_sell) = (vec![0; n], vec![0;n]);
        dp_buy[0] = -prices[0]; dp_buy[1] = max(-prices[0], -prices[1]);
        dp_sell[0] = 0; dp_sell[1] = max(0, prices[1] - prices[0]);
        for i in 2..n {
            dp_buy[i] = max(dp_sell[i - 2] - prices[i], dp_buy[i - 1]);
            dp_sell[i] = max(dp_buy[i - 1] + prices[i], dp_sell[i - 1]);
        }
        max(dp_buy[n - 1], dp_sell[n - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::max_profit(vec![1,2,3,0,2]));
    }
}
