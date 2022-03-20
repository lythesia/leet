/**
 * [322] Coin Change
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
You may assume that you have an infinite number of each kind of coin.
 
Example 1:
Input: coins = [1,2,5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1

Example 2:
Input: coins = [2], amount = 3
Output: -1

Example 3:
Input: coins = [1], amount = 0
Output: 0

 
Constraints:

	1 <= coins.length <= 12
	1 <= coins[i] <= 231 - 1
	0 <= amount <= 104

 */
pub struct Solution {}

// submission codes start here

use std::cmp::min;
impl Solution {
    // dp[i][j]: 前i种硬币, 换取价值j的最小硬币数
    // dp[_][0] = 0
    // dp[i][j] = min { dp[i - 1][j - k*c[i]] + k} for all k in 0..j/k
    //          = min { dp[i - 1][j], dp[i][j - c[i]] + 1}
    // compressed: dp[j] = min { dp[j], dp[j - ci] + 1 }
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![0x3f3f3f3f; amt + 1];
        dp[0] = 0;
        for c in coins {
            for j in c as usize..=amt {
                dp[j] = min(dp[j], dp[j - c as usize] + 1);
            }
        }
        if dp[amt] != 0x3f3f3f3f {
            dp[amt]
        } else { -1 }
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
