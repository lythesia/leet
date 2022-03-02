/**
 * [746] Min Cost Climbing Stairs
 *
 * You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
You can either start from the step with index 0, or the step with index 1.
Return the minimum cost to reach the top of the floor.
 
Example 1:
Input: cost = [10,15,20]
Output: 15
Explanation: You will start at index 1.
- Pay 15 and climb two steps to reach the top.
The total cost is 15.

Example 2:
Input: cost = [1,100,1,1,1,100,1,1,100,1]
Output: 6
Explanation: You will start at index 0.
- Pay 1 and climb two steps to reach index 2.
- Pay 1 and climb two steps to reach index 4.
- Pay 1 and climb two steps to reach index 6.
- Pay 1 and climb one step to reach index 7.
- Pay 1 and climb two steps to reach index 9.
- Pay 1 and climb one step to reach the top.
The total cost is 6.

 
Constraints:

	2 <= cost.length <= 1000
	0 <= cost[i] <= 999

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // f1 = c0
    // f2 = min(c0, c1)
    // fn: if from n-1 to n, fn-1 + cn-1
    //     or from n-2 to n, fn-2 + cn-2
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        use std::cmp::min;
        let n = cost.len();
        match n {
            0|1 => 0,
            2 => { min(cost[0], cost[1]) },
            n => {
                let (mut a, mut b) = (0, min(cost[0], cost[1]));
                for i in 3..=n {
                    let f = min(a + cost[i-2], b + cost[i-1]);
                    a = b;
                    b = f;
                }
                b
            },
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10,15,20]));
        assert_eq!(6, Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]));
    }
}
