/**
 * [518] Coin Change 2
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
You may assume that you have an infinite number of each kind of coin.
The answer is guaranteed to fit into a signed 32-bit integer.
 
Example 1:
Input: amount = 5, coins = [1,2,5]
Output: 4
Explanation: there are four ways to make up the amount:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1

Example 2:
Input: amount = 3, coins = [2]
Output: 0
Explanation: the amount of 3 cannot be made up just with coins of 2.

Example 3:
Input: amount = 10, coins = [10]
Output: 1

 
Constraints:

	1 <= coins.length <= 300
	1 <= coins[i] <= 5000
	All the values of coins are unique.
	0 <= amount <= 5000

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i][j]: 前i种硬币, 换取价值j的组合数
    // dp[_][0] = 1
    // dp[i][j] = sum{ dp[i - 1][j - k*c[i]] } for all k in 0..=j/c[i]
    // dp[i, j - c[i]] = sum = { dp[i - 1][j - c[i] - k*c[i]] } for all k in 0..=j/c[i]-1
    // dp[i][j] = dp[i - 1][j] + dp[i, j - c[i]]
    // compressed: dp[j] = dp[j] + dp[j - c[i]]
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![0; amt + 1];
        dp[0] = 1;
        for c in coins {
            let ci = c as usize;
            for j in ci..=amt {
                dp[j] += dp[j - ci];
            }
        }
        dp[amt]
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
