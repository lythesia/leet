/**
 * [365] Water and Jug Problem
 *
 * You are given two jugs with capacities jug1Capacity and jug2Capacity liters. There is an infinite amount of water supply available. Determine whether it is possible to measure exactly targetCapacity liters using these two jugs.
If targetCapacity liters of water are measurable, you must have targetCapacity liters of water contained within one or both buckets by the end.
Operations allowed:

	Fill any of the jugs with water.
	Empty any of the jugs.
	Pour water from one jug into another till the other jug is completely full, or the first jug itself is empty.

 
Example 1:
Input: jug1Capacity = 3, jug2Capacity = 5, targetCapacity = 4
Output: true
Explanation: The famous Die Hard example 

Example 2:
Input: jug1Capacity = 2, jug2Capacity = 6, targetCapacity = 5
Output: false

Example 3:
Input: jug1Capacity = 1, jug2Capacity = 2, targetCapacity = 3
Output: true

 
Constraints:

	1 <= jug1Capacity, jug2Capacity, targetCapacity <= 106

 */
pub struct Solution {}

// submission codes start here

fn gcd(x: i32, y: i32) -> i32 {
    if x > y {
        gcd(y, x)
    } else {
        if y % x == 0 { x }
        else { gcd(y % x, x) }
    }
}

impl Solution {
    // 不断从j1加满, 倒入j2, j2满了就倒空(j1剩余的再倒入)
    // 本质上其实是总共m个j1倒给j2,不断对j2取模, 看最终是否得到目标t
    // 即 m*j1 % j2 === t, m*j1 和 t 对j2同余
    // 或者表示为 m*j1 + n*j2 == t (m, n为整数, 可为负)
    // 根据扩展欧几里得可知, gcd(j1, j2)为t因数即可
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        target_capacity <= jug1_capacity + jug2_capacity && target_capacity % gcd(jug1_capacity, jug2_capacity) == 0
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
