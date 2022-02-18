/**
 * [1711] Count Good Meals
 *
 * A good meal is a meal that contains exactly two different food items with a sum of deliciousness equal to a power of two.
You can pick any two different foods to make a good meal.
Given an array of integers deliciousness where deliciousness[i] is the deliciousness of the i​​​​​​th​​​​​​​​ item of food, return the number of different good meals you can make from this list modulo 109 + 7.
Note that items with different indices are considered different even if they have the same deliciousness value.
 
Example 1:
Input: deliciousness = [1,3,5,7,9]
Output: 4
Explanation: The good meals are (1,3), (1,7), (3,5) and, (7,9).
Their respective sums are 4, 8, 8, and 16, all of which are powers of 2.

Example 2:
Input: deliciousness = [1,1,1,3,3,3,7]
Output: 15
Explanation: The good meals are (1,1) with 3 ways, (1,3) with 9 ways, and (1,7) with 3 ways.
 
Constraints:

	1 <= deliciousness.length <= 105
	0 <= deliciousness[i] <= 220

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        const M: i64 = 10_i64.pow(9) + 7;
        let mut ret = 0_i64;
        let mut h = HashMap::new();
        let powers: Vec<i32> = (0..22).map(|x| 2_i32.pow(x)).collect();

        for x in &deliciousness {
            for p in &powers {
                if let Some(c) = h.get(&(p - x))  {
                    ret += c;
                }
            }
            *h.entry(*x).or_insert(0) += 1;
        }

        (ret % M) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::count_pairs(vec![1,3,5,7,9]));
        assert_eq!(15, Solution::count_pairs(vec![1,1,1,3,3,3,7]));
    }
}
