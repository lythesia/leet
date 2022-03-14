/**
 * [96] Unique Binary Search Trees
 *
 * Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
 
Example 1:
Input: n = 3
Output: 5

Example 2:
Input: n = 1
Output: 1

 
Constraints:

	1 <= n <= 19

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // 对有n节点的tree:
    // n = 1, a[1] = 1
    // n = 2, 以1为根=1, 以2为根=1, a[2] = 1 + 1
    // n = 3, 以1为根, [2,3]只能为右子树即=a[2]; 以2为根, [1]为左子树, [2]为右子树=a[1]*a[1]; 以3为根, 同1, =a[2]. tot = a[2] + a[1]*a[1] + a[2]
    // n, 以1为根, []为左子树, [2..=n]为右子树, a[0]*a[n-1]; 以2为根, [1]为左, [3..=n]为右, a[1]*a[n-2], 以此类推
    // a[n] = sigma(i <- 0..n-1) a[i]*a[n-1-i]
    // p.s 其实就是卡特兰数
    // C(n) = (2n)! / ( n! * (n + 1)! )
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1; dp[1] = 1;
        for k in 2..=n {
            for i in 0..=k-1 {
                dp[k] += dp[i]*dp[k-1-i];
            }
        }
        dp[n]
    }

    fn catalan(n: i32) -> i32 {
        // 1*2*..2n / (1*2..n) / (1*2*3..n+1) => (n+1 * n+2 .. 2n) / (1 * 2 * 3 .. n) / (n + 1)
        let n = n as i64; // avoid overflow
        ((1..=n).fold(1, |acc, i| acc * (n + i) / i) / (n + 1)) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::num_trees(3));
        assert_eq!(1, Solution::num_trees(1));
    }
}
